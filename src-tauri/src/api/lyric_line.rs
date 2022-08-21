use crate::get_lyrics::lyric_file::activate_lyric;
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

    let res: Result<Lrcx, anyhow::Error> = activate_lyric(&player_info).await;
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
    use super::get_next_inline_lyrics;

    #[test]
    fn test_whole() {
        //create an tokio async runtime to run get_next_inline_lyrics function
        let mut runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let result = get_next_inline_lyrics(0.0).await;
            println!("result{}", result);
        });
    }
}
