use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use anyhow::Result as AnyResult;
use directories::UserDirs;
use log::info;

use crate::api::model::Lrcx;
use crate::get_lyrics::netease::get_lyrics::get_lyric_file;
use crate::player_info::link_system::PlayerInfo;


pub fn lyric_file_path(song: &PlayerInfo) -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let document_path = user_dirs.document_dir().unwrap();
    let lyrecs_path = document_path.join("Lyrecs");
    if lyrecs_path.exists() {
        if !lyrecs_path.is_dir() {
            panic!("lyrecs path is not a directory");
        }
    } else {
        std::fs::create_dir_all(lyrecs_path.clone()).unwrap();
    }
    let file_name = format!("{} - {}.lrcx", song.artist, song.title);
    lyrecs_path.join(file_name)
}


pub fn lyric_file_exists(song: &PlayerInfo) -> bool {
    lyric_file_path(song).exists()
}

pub async fn activate_lyric(song: &PlayerInfo) -> AnyResult<Lrcx> {
    info!("getting lyric for {}, artist {}", song.title, song.artist);
    let lyric_str = get_lyric_file(song).await.unwrap();
    if lyric_str.is_empty() {
        return Err(anyhow::anyhow!("lyric file is empty"))
    }
    let lrc = serde_json::from_str::<Lrcx>(&lyric_str)?;
    Ok(lrc)
}
