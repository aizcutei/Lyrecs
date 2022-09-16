use crate::get_lyrics::song::Song;
use anyhow::Result as AnyResult;
use log::{info, warn};
use serde_json::Value;
use std::process;

#[derive(Debug, Default, Clone)]
pub struct PlayerInfo {
    pub track: Song,
    pub state: String,
    pub duration: f64,
    pub position: f64,
}

#[cfg(target_os = "macos")]
pub async fn get_player_info() -> AnyResult<PlayerInfo> {
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
        var album = track.album();
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

        let player_info_result: Value = serde_json::from_slice(&query_playing_result.stdout)
            .map_or_else(
                |err| {
                    warn!("No song is playing or Music is not running{}", err);
                    Value::Null
                },
                |player_info| player_info,
            );
        if player_info_result.is_null() {
            return Err(anyhow::anyhow!(
                "No song is playing or Music is not running"
            ));
        }

        let player_info = PlayerInfo {
            track: Song {
                title: player_info_result["title"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                artist: player_info_result["artist"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                album: player_info_result["album"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            },
            state: player_info_result["state"].as_str().unwrap().to_string(),
            duration: player_info_result["duration"].as_f64().unwrap(),
            position: player_info_result["position"].as_f64().unwrap(),
        };

        return Ok(player_info);
    }
    Err(anyhow::anyhow!("Apple Music is not running"))
}

#[cfg(target_os = "windows")]
pub async fn get_player_info() -> AnyResult<PlayerInfo> {
    use anyhow::Ok;

    use super::windows::ITunes;

    let mut itunes = ITunes::new()?;
    let track_info = itunes.get_current_track_info();
    if track_info.is_none() {
        return Err(anyhow::anyhow!(
            "No song is playing or iTunes is not running"
        ));
    }
    let track_info = track_info.unwrap();
    let position = itunes.get_player_position().unwrap();

    Ok(PlayerInfo {
        track: Song {
            title: track_info.name,
            artist: track_info.artist,
            album: track_info.album,
        },
        state: itunes.is_playing().to_string(),
        duration: track_info.duration as f64,
        position: position.as_secs_f64(),
    })
}

#[cfg(target_os = "windows")]
pub async fn get_player_info_desktop() {
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
        use log::warn;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let a = get_player_info().await;
                match a {
                    Ok(res) => {
                        println!(
                            "{} {} {} {} {} {}",
                            res.track.title,
                            res.track.album,
                            res.track.artist,
                            res.duration,
                            res.position,
                            res.state,
                        );
                    }
                    Err(err) => {
                        println!("err{}", err.to_string())
                    }
                }
            });
        // let player_info = get_player_info();
    }
}
