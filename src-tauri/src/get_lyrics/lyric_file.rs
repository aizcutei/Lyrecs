use std::fs::File;
use std::io::{Write, Read, BufWriter};
use std::path::PathBuf;
use anyhow::Result as AnyResult;
use directories::UserDirs;
use env_logger::fmt::BufferWriter;
use log::info;
use serde_json::json;
use crate::api::lyric_line::{LyricTimeLine, LyricLine};
use crate::get_lyrics::kugou;
use crate::get_lyrics::netease::{get_song_lyric, get_best_match_song, parse_netease_lyric};
use crate::get_lyrics::song_struct::{NeteaseSong, Song, KugouSong};
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


fn lyric_file_exists(song: &PlayerInfo) -> bool {
    lyric_file_path(song).exists()
}


pub async fn save_lyric_file(song: &PlayerInfo) -> AnyResult<()> {
    info!("getting default song");

    let mut search_song = NeteaseSong::new_empty();
    search_song.name = song.title.clone().replace('&', "");
    search_song.artist = song.artist.clone().replace('&', "");
    //remove & in the keyword

    let default_song = get_best_match_song(&search_song).await;
    info!("default song {} \n getting lyric", default_song.name);

    let song_lyrics = get_song_lyric(&default_song).await?;
    info!("song_lyrics {:?}", song_lyrics);

    let mut lrcx = parse_netease_lyric(song_lyrics.get_original_lyric().unwrap(), "\n")?;
    info!("writing lyric file of length {}", lrcx.lyric_body.len());

    let mut file = File::create(lyric_file_path(song))?;
    
    let mut default_timeline: LyricTimeLine = Default::default();
    if lrcx.lyric_body.len() == 0 {
        default_timeline.line.set_text("No Lyric for this song".to_string());
    } 
    default_timeline.time = "00:00.000".to_string();
    lrcx.lyric_body.insert(0, default_timeline);
    let serial = serde_json::to_string(&lrcx)?;
    write!(file, "{}", serial)?;

    Ok(())
}

pub async fn kugou_save_lyric_file(song: &PlayerInfo) -> AnyResult<()> {
    info!("getting default song");

    //remove & in the keyword
    let mut search_song = format!("{} {}", song.title.clone().replace('&', ""), song.artist.clone().replace('&', ""));

    let default_song = kugou::get_default_song(&search_song).await;
    info!("default song {:?} \n getting lyric", default_song.name);

    let lyrics_list  =kugou::get_lyrics_list(&default_song).await.unwrap();

    let song_lyrics = kugou::get_default_lyric_item(&lyrics_list).await;

    let lyric_str = kugou::get_song_lyric(&song_lyrics).await.unwrap();

    //let lrcx = Lrcx::from_str(song_lyrics.get_original_lyric().unwrap(), "\n").unwrap();
    info!("writing lyric file of length");

    let mut file = File::create(lyric_file_path(song))?;
    /* 
    if lrcx.is_empty() {
        file.write_all(b"[00:00.000] No Lyric for this song\n[00:10.000] \xE2\x99\xAB ~ ~ ~")?; //add a start line
    } else {
        file.write_all(b"[00:00.000] \n")?; //add a start line
    }
    
    
    for line in lrcx.iter() {
        file.write_all(line.to_string().as_bytes())?;
        file.write_all(b"\n")?;
    }
    */
    file.write_all(lyric_str.decoded.as_bytes())?;

    Ok(())
}

pub async fn get_lyric_file(song: &PlayerInfo) -> AnyResult<String> {
    if !lyric_file_exists(song) {
        info!("lyric file does not exist");
        save_lyric_file(song).await?;
    }
    let lyric_path = lyric_file_path(song);
    let mut file = File::open(lyric_path)?;
    let mut lyric = String::new();
    file.read_to_string(&mut lyric)?;
    Ok(lyric)
}

pub async fn activate_lyric(song: &PlayerInfo) -> AnyResult<Lrcx> {
    info!("getting lyric for {}, artist {}", song.title, song.artist);
    let lyric_str = get_lyric_file(song).await.unwrap();
    if lyric_str.is_empty() {
        return Err(anyhow::anyhow!("lyric file is empty"))
    }
    Lrcx::from_str(lyric_str, "\n")
}