use serde::Serialize;
use serde_json::json;

use crate::{
    cache::CacheManager,
    get_lyrics::{
        kugou::{self, get_lyrics::get_song_lyric as k_get_song_lyric},
        lyric_file::LyricSource,
        netease,
        netease::get_lyrics::get_song_lyric as n_get_song_lyric,
    },
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct SearchRes {
    pub title: String,
    pub artist: String,
    pub source: String,
}

type SearchResponse = Vec<SearchRes>;

lazy_static! {
    static ref current_search: CacheManager<Vec<LyricSource>> = CacheManager::new();
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
                .push(LyricSource::Kugou(song.clone()));
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
                .push(LyricSource::Netease(song.clone()));
        }
    }
    json!(rsp).to_string()
}
#[tauri::command]
pub async fn search_lyric(index: usize) -> String {
    let song = &(&*current_search.get_cache().await)[index];
    match song {
        LyricSource::Netease(song) => match n_get_song_lyric(song).await {
            Ok(l) => l.lyric,
            Err(_) => String::new(),
        },
        LyricSource::Kugou(song) => match k_get_song_lyric(song).await {
            Ok(l) => l.decoded,
            Err(e) => e.to_string(),
        },
    }
}
