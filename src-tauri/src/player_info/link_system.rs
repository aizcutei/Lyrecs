
use std::process;
use anyhow::Result as AnyResult;
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct PlayerInfo {
   pub state: String,
   pub title: String,
   pub artist: String,
   pub album: String,
   pub duration: i64,
   pub position: i64,
}


#[cfg(target_os = "macos")]
pub fn get_player_info() -> AnyResult<Value> {
    let query_running_status_script = r#"
        var Sys = Application("System Events")
        var running = Sys.processes['Music'].exists()
        runnning"#;

    let query_playing_status_script = r#"
        var Music = Application('Music');
        var time = Music.playerPosition();
        var track = Music.currentTrack();
        var title = track.name();
        var artist = track.artist();
        var album = track.album;
        var duration = track.duration();
        var state = Music.playerState();
        var obj = {state:state,position:time,title:title,duration:duration,artist:artist,album:album};
        JSON.stringify(obj)"#;

    let query_running_result = process::Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(&query_running_status_script)
        .output()
        .expect("OSA process failed to execute");

    if query_running_result.stdout != [116, 114, 117, 101, 10] {
        let query_playing_result = process::Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(&query_playing_status_script)
        .output()
        .expect("OSA process failed to execute");

        let player_info: Value = serde_json::from_slice(&query_playing_result.stdout)
            .expect("Failed to parse player info");

        Ok(player_info)
    } else {
        return Err(anyhow::anyhow!("Apple Music is not running"));
    }
}


#[cfg(target_os = "windows")]
 pub async fn get_player_info() -> AnyResult<PlayerInfo> {
    use windows::Media::Control;
    /* UWP API, desktop API in investigation */
    let session = Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync();
    let current_session = &(session?.await?.GetCurrentSession()?);
    let track_info = &current_session.TryGetMediaPropertiesAsync()?.await?;
    let timeline = &current_session.GetTimelineProperties()?;
    let status = &current_session.GetPlaybackInfo()?.PlaybackStatus()?.0;
    
   Ok(PlayerInfo {
        state: status.to_string(),
        title: track_info.Title()?.to_string(),
        artist: track_info.Artist()?.to_string(),
        album: track_info.AlbumTitle()?.to_string(),
        duration: timeline.EndTime()?.Duration - timeline.StartTime()?.Duration,
        position: timeline.Position()?.Duration,
    })
}

#[cfg(target_os = "windows")]
pub async fn get_player_info_desktop(){
    unimplemented!()
}

#[cfg(target_os = "linux")]
pub fn get_player_info() -> AnyResult(PlayerInfo) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_player_info() {
        use log::{ warn};
        println!("asdsadasda");
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                println!("ttttttt");
                let a = get_player_info().await;
                match a {
                    Ok(res) => {println!("title{} {} {}", res.title, res.album, res.artist);},
                    Err(err) => {println!("err{}", err.to_string())},
                }
                println!("sssssss");
                warn!("asdasdad");
            });
        // let player_info = get_player_info();
        
    }
}