use crate::{
    cache::CacheManager,
    get_lyrics::{
        cache::get_cache_manager as get_lyrcx_cache_manager,
        kugou::{self, get_lyrics::get_song_lyric as k_get_song_lyric},
        lyric_file::{lyric_file_path, write_lyric_file, LyricSource},
        netease::get_lyrics::get_song_lyric as n_get_song_lyric,
        netease::{self},
    },
    player_info::{self, link_system::get_player_info},
};

use serde::Serialize;
use serde_json::json;

use super::model::Lrcx;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SearchRes {
    pub title: String,
    pub artist: String,
    pub source: String,
}

type SearchResponse = Vec<SearchRes>;

lazy_static! {
    static ref current_search: CacheManager<Vec<LyricSource>> = CacheManager::new();
    static ref current_content: CacheManager<Lrcx> = CacheManager::new();
}

#[tauri::command]
pub async fn search(content: String) -> String {
    // is it enough to use only title
    let kugou = kugou::get_lyrics::get_song_list(&content, 5).await;
    let netease = netease::get_lyrics::get_song_list(&content, 5).await;
    let mut rsp = SearchResponse::with_capacity(10);
    if let Ok(k) = kugou {
        for song in k.0 {
            rsp.push(SearchRes {
                title: song.name.clone(),
                artist: song.artist.clone(),
                source: "Kugou".to_string(),
            });
            current_search
                .get_cache()
                .await
                .push(LyricSource::Kugou(song));
        }
    }
    if let Ok(n) = netease {
        for song in n.0 {
            rsp.push(SearchRes {
                title: song.name.clone(),
                artist: song.artist.clone(),
                source: "Netease".to_string(),
            });
            current_search
                .get_cache()
                .await
                .push(LyricSource::Netease(song));
        }
    }
    json!(rsp).to_string()
}
#[tauri::command]
pub async fn search_lyric(index: usize) -> Result<String, String> {
    let song = &(current_search.get_cache().await)[index];
    current_content.set_fresh(false).await;
    let mut preview = String::new();
    let lyric = match song {
        LyricSource::Netease(song) => match n_get_song_lyric(song).await {
            Ok(l) => {
                preview = l.lyric.clone();
                l.to_lrcx()
            }
            Err(e) => return Err(e.to_string()),
        },
        LyricSource::Kugou(song) => match k_get_song_lyric(song).await {
            Ok(l) => {
                preview = l.decoded.clone();
                l.to_lrcx()
            }
            Err(e) => return Err(e.to_string()),
        },
    };
    current_content.set_cache(lyric).await;
    Ok(preview)
}

#[tauri::command]
pub async fn apply_search_result(index: usize) -> Result<(), String> {
    if !current_content.is_fresh().await {
        search_lyric(index).await?;
    }
    let lrcx = current_content.get_cache().await.clone();
    let lrcx_clone = lrcx.clone();
    get_lyrcx_cache_manager().set_cache(lrcx_clone).await;
    let info = get_player_info().await;
    if let Ok(current_info) = info {
        let res = write_lyric_file(
            lyric_file_path(&current_info.track.artist, &current_info.track.title),
            lrcx,
        );
        if let Err(res) = res {
            return Err(res.to_string());
        }
        return Ok(());
    }
    Err(info.err().unwrap().to_string())
}
