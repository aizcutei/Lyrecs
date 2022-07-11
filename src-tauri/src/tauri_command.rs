use serde_json::{json, Value, Map};
use crate::{link_system::get_player_info, player_info};
use crate::get_lyrics::{song_struct::Song};
use crate::get_lyrics::netease::{self, get_default_song};
use crate::get_lyrics::lyric_file::{set_lyric_file, get_lyric_file};
use crate::parse_lyric::parser;

#[tauri::command]
pub fn connect_test(text: &str) -> String {
    format!("Hello {}!", text)
}

#[tauri::command]
pub fn get_next_inline_lyric() -> String {
    let player_info = get_player_info().unwrap();
    let song_name = player_info["title"].as_str().unwrap();
    let artist_name = player_info["artist"].as_str().unwrap();
    let song_info = format!("{} - {}", song_name, artist_name);
    let song = get_default_song(&song_info);
    let lrc = parser::active_lyric(song);
    let time = player_info["position"].as_f64().unwrap();
    let lyric_inline = parser::get_lyric_inline(&lrc, time);

    lyric_inline.unwrap()
}

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

