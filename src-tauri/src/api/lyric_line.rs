use crate::get_lyrics::kugou::model::KugouSong;
use crate::get_lyrics::lyric_file::LyricSource;
use crate::get_lyrics::{lyric_file::activate_lyric, netease::model::NeteaseSong};
use crate::player_info::link_system::get_player_info;
use log::{warn};
use serde_json::{json};
use crate::api::model::{Lrcx};

#[tauri::command]
pub async fn get_next_inline_lyrics(fix_time: f64) -> String {
    let mut player_info = match get_player_info().await{
        Ok(info) => (info),
        Err(err) => {
            warn!("error: {}", err);
            return Default::default()
        },
    };
    // kugou_save_lyric_file(&player_info).await;
    //make sure player position is positive
    if player_info.position >= fix_time.abs() {
        player_info.position += fix_time;
    }
    let n = NeteaseSong{
        name:player_info.track.title.clone(),
        artist:player_info.track.artist.clone(),
        album:player_info.track.album.clone(),
        id: "".to_string(),
    };
    let res: Result<Lrcx, anyhow::Error> = activate_lyric(LyricSource::Netease(n)).await;
    json!(res.map_or_else(
            |err|{
                warn!("error: {}", err);
                Default::default()
            },
            |lrc| {
                lrc.get_next_lyric_by_time(player_info.position)
            })
        ).to_string()
}

#[cfg(test)]
mod tests {
    use log::warn;
    use reqwest::Client;
    use serde_json::json;

    use crate::{api::model::Lrcx, get_lyrics::{lyric_file::{activate_lyric, LyricSource, get_client_provider}, netease::model::NeteaseSong, kugou::model::KugouSong, cache::get_cache_manager}, player_info::link_system::get_player_info};

    #[test]
    fn test_whole() {
        //create an tokio async runtime to run get_next_inline_lyrics function
        let mut runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
        let _lyric_cache = tokio::spawn(get_cache_manager().update());

        // 测试需要代理就在这里加，如果要跑一堆测试记得这里完了过后去掉代理
        get_client_provider().set(
            Client::builder()
            .proxy(reqwest::Proxy::http("http://127.0.0.1:7890").unwrap())
            .proxy(reqwest::Proxy::https("https://127.0.0.1:7890").unwrap())
            .build().unwrap()
        ).await;
        let mut player_info = match get_player_info().await{
            Ok(info) => (info),
            Err(err) => {
                warn!("error: {}", err);
                return Default::default()
            },
        };
        let n = NeteaseSong{
            name:player_info.track.title.clone(),
            artist:player_info.track.artist.clone(),
            album:player_info.track.album.clone(),
            id: "".to_string(),
        };

        let k = KugouSong{
            name: player_info.track.title.clone(),
            artist: player_info.track.artist.clone(),
            album: player_info.track.album.clone(),
            hash: Default::default(),
            id: Default::default(),
            access_key: Default::default(),
            };

        let res: Result<Lrcx, anyhow::Error> = activate_lyric(LyricSource::Kugou(k)).await;
        let res = json!(res.map_or_else(
            |err|{
                println!("error: {}", err);
                Default::default()
            },
            |lrc| {
                lrc.get_next_lyric_by_time(player_info.position)
            })
        ).to_string();
            // let result = get_next_inline_lyrics(0.0).await;
            println!("result{}", res);
        });
    }
}
