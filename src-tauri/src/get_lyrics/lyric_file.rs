use std::fs::File;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use anyhow::Result as AnyResult;
use directories::UserDirs;
use log::info;
use crate::get_lyrics::song_struct::{Song, SongList, SongLyrics};
use crate::get_lyrics::netease::get_song_lyric;
use crate::parse_lyric::lrcx_parser::Lrcx;
use crate::player_info::link_system::PlayerInfo;

use super::netease::get_default_song;

fn lyric_file_path(song: &PlayerInfo) -> PathBuf {
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

fn find_lyric_file(song: &PlayerInfo) -> AnyResult<bool> {
    let lyric_path = lyric_file_path(song);
    let if_exist = lyric_path.exists();
    Ok(if_exist)
}


pub async fn save_lyric_file(song: &PlayerInfo) -> AnyResult<String> {
    let if_exist = find_lyric_file(song).unwrap();
    let lyric_path = lyric_file_path(song);
    let mut file = File::create(lyric_path)?;
    if if_exist {
        let mut lyric = String::new();
        file.read_to_string(&mut lyric)?;
        Ok(lyric)
    } else {
        info!("getting default song");
        let default_song = get_default_song(&song.title).await;
        info!("default song {} \n getting lyric", default_song.name);
        let song_lyrics = get_song_lyric(&default_song).await.unwrap();
        info!("song_lyrics {:?}", song_lyrics);
        let lrcx = Lrcx::from_str(song_lyrics.get_original_lyric().unwrap(), "\\n").unwrap();
        info!("writing lyric file of length {}", lrcx.iter().len());
        for line in lrcx.iter() {
            file.write_all(line.to_string().as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(song_lyrics.get_original_lyric().unwrap())
    }
}


pub async fn get_lyric_file(song: &PlayerInfo) -> AnyResult<String> {
    let exists = find_lyric_file(song).unwrap();
    info!("Lyric file {}", {if exists {"exists"} else {"not exists"}});
    if exists {
        let lyric_path = lyric_file_path(song);
        let mut file = File::open(lyric_path)?;
        let mut lyric = String::new();
        file.read_to_string(&mut lyric)?;
        Ok(lyric)
    } else {
        Ok(save_lyric_file(song).await.unwrap())
    }
}