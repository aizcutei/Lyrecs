use std::rc::Rc;
use anyhow::Result as AnyResult;

use crate::{player_info::link_system, get_lyrics::{song_struct::Song, lyric_file::get_lyric_file}};
use lrc::{Lyrics, IDTag, TimeTag};


pub fn active_lyric(song: Song) -> Lyrics {
    let lyric_str = get_lyric_file(&song).unwrap();

    let lyrics = Lyrics::from_str(&lyric_str).unwrap();

    lyrics
}

fn time_f64_to_time_tag(time: f64) -> TimeTag {
    let time_mm = (time.floor()/60.0).floor();
    let time_ss = time.floor()%60.0;
    let time_ms = time.fract()*1000.0;
    let total_ms = ((time_mm*60.0 + time_ss)*1000.0 + time_ms.floor()) as i64;
    //let time_str = format!("0{}:{}.{}", time_mm, time_ss, time_ms.floor());
    let time_tag = TimeTag::new(total_ms);
    time_tag
}

pub fn get_lyric_inline(lyrics: &Lyrics, time: f64) -> AnyResult<String> {
    let time_tag = time_f64_to_time_tag(time);
    
    let mut lyric_inline = String::new();

    if let Some(index) = lyrics.find_timed_line_index(time_tag) {
        let timed_lines = lyrics.get_timed_lines();
        let (time_inline, lrc_inline) = timed_lines.get(index).unwrap().clone();
        lyric_inline = lrc_inline.to_string();
    } else {
        lyric_inline = "Error when parse lyric".to_string();
    }
    Ok(lyric_inline)
}