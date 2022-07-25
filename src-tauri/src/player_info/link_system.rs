
use std::process;
use anyhow::Result as AnyResult;
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct PlayerInfo {
    pub state: String,
    pub title: String,
    pub artist: String,
    pub album: String,
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
            .expect("Failed to parse player info, No song is playing or Music is not running");


        let player_info = PlayerInfo {
            state: player_info_result["state"].as_str().unwrap().to_string(),
            title: player_info_result["title"].as_str().unwrap().to_string(),
            artist: player_info_result["artist"].as_str().unwrap().to_string(),
            album: player_info_result["album"].as_str().unwrap().to_string(),
            duration: player_info_result["duration"].as_f64().unwrap(),
            position: player_info_result["position"].as_f64().unwrap(),
        };

        Ok(player_info)
    } else {
        return Err(anyhow::anyhow!("Apple Music is not running"));
    }
}


#[cfg(target_os = "windows")]
pub async fn get_player_info() -> AnyResult<PlayerInfo> {
    use std::{io::ErrorKind};

    use anyhow::Ok;
    use windows::Media::Control;
    /* UWP API, desktop API in investigation */
    let session = Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync();
    let current_session = &(session?.await?.GetCurrentSession()?);
    if current_session.SourceAppUserModelId()?.to_string() != get_app_id_by_name("iTunes") {
        return Err(anyhow::Error::new(std::io::Error::new(ErrorKind::Other, "itunes not running")))
    };
    let track_info = &current_session.TryGetMediaPropertiesAsync()?.await?;
    let timeline = &current_session.GetTimelineProperties()?;
    let status = &current_session.GetPlaybackInfo()?.PlaybackStatus()?.0;
    
    Ok(PlayerInfo {
        state: status.to_string(),
        title: track_info.Title()?.to_string(),
        artist: track_info.Artist()?.to_string(),
        album: track_info.AlbumTitle()?.to_string(),
        duration: timeline.EndTime()?.Duration as f64 - timeline.StartTime()?.Duration as f64,
        position: timeline.Position()?.Duration as f64,
    })
}

#[cfg(target_os = "windows")]
fn get_app_id_by_name(name: &str ) -> String {
    use std::process::Command;
    use std::{str, fs};
    let args:[&str; 8] = ["-C", "get-startapps", "iTunes", "|" , "out-file","out.txt", "-encoding", "utf8"];
    Command::new("powershell")
        .args(args)
        .output()
        .expect("Failed to execute powershell command");
    let res = fs::read_to_string("out.txt").expect("Something went wrong reading the file");
    res.split('\n').find(|line| line.starts_with(name))
                    .and_then(|s| s.strip_prefix(name))
                    .map_or(String::new(), |s| String::from(s.trim()))
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

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_app_id() {
        println!("{}",get_app_id_by_name("iTunes"));
    }
}