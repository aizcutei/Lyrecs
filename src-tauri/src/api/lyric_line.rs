use crate::get_lyrics::lyric_file::{activate_lyric, kugou_save_lyric_file};
use crate::player_info::link_system::get_player_info;
use log::{warn, info};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use core::fmt;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use hhmmss::Hhmmss;

extern crate chrono;

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone, Serialize, Deserialize)]
pub struct IDTag {
    tag_name: String,
    tag_value: String,
}

impl IDTag {
    pub fn new(tag_name: String, tag_value: String) -> IDTag {
        IDTag {
            tag_name,
            tag_value,
        }
    }
}

impl Display for IDTag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]{}", self.tag_name, self.tag_value)
    }
}

#[derive(Debug, Clone, Default,Serialize, Deserialize)]

pub struct Lrcx {
    pub metadata: BTreeSet<IDTag>,
    pub lyric_body: Vec<LyricTimeLine>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LyricTimeLine {
    pub time: String,
    pub line: LyricLine
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LyricLine {
    pub text : String,
    pub length: i64,
    pub word_timeline : Vec<WordTimeline>,
    pub translation: String,
    pub pronunciation: String,
}

impl LyricLine {
    pub fn set_text(&mut self, text: String)  {
        self.text = text;
    }
    pub fn set_translation(&mut self, translation: String)  {
        self.translation = translation;
    }
    pub fn set_pronunciation(&mut self, pronunciation: String)  {
        self.pronunciation = pronunciation;
    }
    pub fn set_word_timeline(&mut self, word_timeline: &Vec<WordTimeline>)  {
        self.word_timeline = word_timeline.to_vec();
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WordTimeline{
    pub pos: i64,
    pub start: i64,
    pub end: i64,
}

pub trait LyricLineResponse {
    fn get_lyric_line(&self) -> LyricLine;
}

#[tauri::command]
pub async fn get_next_inline_lyrics(fix_time: f64) -> String {
    let mut player_info = match get_player_info().await{
        Ok(info) => (info),
        Err(err) => {
            warn!("error: {}", err);
            return Default::default()
        },
    };
    kugou_save_lyric_file(&player_info).await;
    //make sure player position is positive
    if player_info.position >= fix_time.abs() {
        player_info.position += fix_time;
    }

    json!(activate_lyric(&player_info).await
        .map_or_else(
            |err|{
                warn!("error: {}", err);
                Default::default()
            },
            |lrc| {            
                lrc.get_next_lyric_by_time(player_info.position)
            })
        ).to_string()
}

impl LyricLine {
    pub fn new(text: String, length: i64, word_timeline: Vec<WordTimeline>, translation: String, pronunciation: String) -> LyricLine {
        LyricLine {
            text,
            length,
            word_timeline,
            translation,
            pronunciation,
        }
    }
}

impl WordTimeline {
    pub fn new(pos: i64, start: i64, end: i64) -> WordTimeline {
        WordTimeline {
            pos,
            start,
            end,
        }
    }
}
