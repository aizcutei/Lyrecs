use std::fs::File;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use anyhow::Result as AnyResult;
use directories::UserDirs;
use crate::get_lyrics::song_struct::{Song, SongList, SongLyrics};
use crate::get_lyrics::netease::get_song_lyric;
use async_recursion::async_recursion;

fn lyric_file_path(song: &Song) -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let document_path = user_dirs.document_dir().unwrap();
    let lyrecs_path = document_path.join("lyrecs");
    let file_name = format!("{} - {}.lrc", song.artist, song.name);
    lyrecs_path.join(file_name)
}

fn find_lyric_file(song: &Song) -> AnyResult<bool> {
    let lyric_path = lyric_file_path(song);
    let if_exist = lyric_path.exists();
    Ok(if_exist)
}


pub fn set_lyric_file(song: &Song) -> AnyResult<String> {
    let if_exist = find_lyric_file(song).unwrap();
    if if_exist {
        Ok(get_lyric_file(song).unwrap())
    } else {
        let lyric_path = lyric_file_path(song);
        let mut file = File::create(lyric_path)?;
        let song_lyrics = get_song_lyric(song).unwrap();
        write!(file, "{}", song_lyrics.to_string())?;
        Ok(song_lyrics.to_string())
    }
}


pub fn get_lyric_file(song: &Song) -> AnyResult<String> {
    let if_exist = find_lyric_file(song).unwrap();
    if if_exist {
        let lyric_path = lyric_file_path(song);
        let mut file = File::open(lyric_path)?;
        let mut lyric = String::new();
        file.read_to_string(&mut lyric)?;
        Ok(lyric)
    } else {
        Ok(set_lyric_file(song).unwrap())
    }
}