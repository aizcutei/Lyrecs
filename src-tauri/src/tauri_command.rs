use crate::get_lyrics::lyric_file::{activate_lyric};
use crate::player_info::link_system::get_player_info;
use log::{warn, info};
use crate::parse_lyric::lrcx_parser::{Lrcx};

#[tauri::command]
pub fn connect_test(text: &str) -> String {
    format!("Hello {}!", text)
}

#[tauri::command]
pub async fn get_next_inline_lyric(fix_time: f64) -> String {
    let mut player_info = match get_player_info().await{
        Ok(info) => (info),
        Err(err) => {
            warn!("error: {}", err);
            return "".to_string()
        },
    };

    //make sure player position is positive
    if player_info.position >= fix_time.abs() {
        player_info.position += fix_time;
    }

    activate_lyric(&player_info).await
    .map_or_else(
        |err|{
            warn!("error: {}", err);
            String::from("")
        },
        |lrc| {
            let time = player_info.position;
            let next_lrc = lrc.get_time_line_by_time(time).unwrap();
            info!("next {}", next_lrc.lyric_str());
            next_lrc.lyric_str()
        })
    
}

//test case

// Old style sync
/* #[tauri::command]
pub fn sync_player_info(now_state: Value) -> Value {
    let player_info = get_player_info().unwrap();
    if (now_state["state"] == player_info["state"]) &&
        (now_state["title"] == player_info["title"]) &&
        (now_state["artist"] == player_info["artist"]) {
            let mut player_info_obj = player_info.as_object().unwrap().to_owned();
            player_info_obj.insert("correct".to_string(), json!(true));
            json!(player_info_obj)
        }else{
            let mut player_info_obj = player_info.as_object().unwrap().to_owned();
            player_info_obj.insert("correct".to_string(), json!(false));
            json!(player_info_obj)
        }
}

#[tauri::command]
pub fn send_default_lyric(song: Value) -> String{
    let default_song = netease::get_default_song(song["name"].as_str().unwrap());
    get_lyric_file(&default_song).unwrap()
} */

