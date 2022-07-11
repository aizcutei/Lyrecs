
use std::process;
use anyhow::Result as AnyResult;
use serde_json::Value;

pub struct PlayerInfo {
    state: String,
    title: String,
    artist: String,
    album: String,
    duration: f64,
    position: f64,
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

//#[cfg(target_os = "windows")]
//use windows::Media::Control;

#[cfg(target_os = "windows")]
pub fn get_player_info() -> AnyResult(PlayerInfo) {
/*     use windows::Media::Control;

    let session =  Control.GlobalSystemMediaTransportControlsSessionManager.RequestAsync();
    let current_session = session.GetCurrentSession();

    if current_session {

    } */

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
        let player_info = get_player_info();
        assert!(player_info.is_ok());
    }
}