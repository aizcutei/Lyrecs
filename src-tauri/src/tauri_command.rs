use serde::{Serialize, Deserialize};



// #[tauri::command]
// pub async fn get_next_inline_lyric_legacy(fix_time: f64) -> String {

//     let mut player_info = match get_player_info().await{
//         Ok(info) => (info),
//         Err(err) => {
//             warn!("error: {}", err);
//             return "".to_string()
//         },
//     };

//     kugou_save_lyric_file(&player_info).await;
//     //make sure player position is positive
//     if player_info.position >= fix_time.abs() {
//         player_info.position += fix_time;
//     }

//     activate_lyric(&player_info).await
//     .map_or_else(
//         |err|{
//             warn!("error: {}", err);
//             String::from("")
//         },
//         |lrc| {
//             let time = player_info.position;
//             let mut passing = Passing_lrc::new_empty();

//             let next_full_lrc = lrc.get_full_time_line_by_time(time).unwrap();
//             info!("next {}", next_full_lrc.len());
//             if next_full_lrc.len() == 0{
//                 passing.origin_lrc = "Can not get next lyric".to_owned();
//             }else if next_full_lrc.len() == 1 {
//                 passing.origin_lrc = next_full_lrc.get(0).unwrap().lyric_str();
//             }else {
//                 for line in next_full_lrc {
//                     if line.lyric_str().starts_with("[tt]"){
//                         let key_frame_str = &line.to_string()[4..];
//                         let key_frame = key_frame_str.split(' ');
//                         unimplemented!()
//                     }
//                 }
//             }

//             serde_json::to_string(&passing).unwrap()
//         })

// }

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

#[derive(Serialize, Deserialize)]
struct Passing_lrc {
    origin_lrc: String,
    translated_lrc: String,
    pronunciation: String,
    key_frame: Vec<f64>,
    is_paused: bool,
}

impl Passing_lrc {
    fn new(origin_lrc: String, translated_lrc: String, pronunciation: String, key_frame: Vec<f64>, is_paused: bool) -> Self {
        Self {
            origin_lrc,
            translated_lrc,
            pronunciation,
            key_frame,
            is_paused,
        }
    }

    fn new_empty() -> Self {
        Self {
            origin_lrc: "".to_string(),
            translated_lrc: "".to_string(),
            pronunciation: "".to_string(),
            key_frame: vec![],
            is_paused: false,
        }
    }
}

