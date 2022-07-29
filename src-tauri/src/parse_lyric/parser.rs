use std::rc::Rc;
use anyhow::Result as AnyResult;
use log::info;

use crate::{player_info::link_system::{self, PlayerInfo}, get_lyrics::{song_struct::Song, lyric_file::get_lyric_file}};
use crate::parse_lyric::lrcx_parser::{Lrcx};


pub async fn activate_lyric(song: &PlayerInfo) -> AnyResult<Lrcx> {
    info!("getting lyric for {}, artist {}", song.title, song.artist);
    let lyric_str = get_lyric_file(song).await.unwrap();
    if lyric_str.is_empty() {
        return Err(anyhow::anyhow!("lyric file is empty"))
    }
    Lrcx::from_str(lyric_str, "\n")
}

pub fn time_f64_to_time_tag(time: f64) -> String {
    let time_mm = (time.floor()/60.0).floor();
    let time_ss = time.floor()%60.0;
    let time_ms = time.fract()*1000.0;
    format!("{}{}:{}.{}", (time_mm as i64)/10, (time_mm as i64) %10, time_ss, time_ms.floor())
}

pub fn time_tag_to_time_f64(time_tag: &str) -> f64 {
    let time_str = time_tag.split(":").collect::<Vec<&str>>();
    let time_mm = time_str[0].parse::<f64>().unwrap();
    let time_ss = time_str[1].parse::<f64>().unwrap();
    time_mm*60.0 + time_ss
}