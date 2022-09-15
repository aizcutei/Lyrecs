use core::fmt;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::Cell;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct Lrcx {
    pub metadata: BTreeSet<IDTag>,
    pub lyric_body: Vec<LyricTimeLine>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LyricTimeLine {
    pub start: f64,
    pub duration: Cell<f64>,
    pub line: LyricLine,
}

impl Lrcx {
    fn find_time_line_index(&self, time: f64) -> Option<usize> {
        let mut index: usize = 0;
        let mut flag = false;
        for (i, line) in self.lyric_body.iter().enumerate() {
            if line.start <= time && line.start + line.duration.get() > time {
                index = i;
                flag = true;
                break;
            }
        }
        if !flag {
            return None;
        }
        Some(index)
    }

    pub fn cal_duration_from_start(&mut self) {
        let mut duration = 0.0;
        if self.lyric_body.len() == 0 {
            return;
        }
        let l = self.lyric_body.len();
        let body = &self.lyric_body;
        let mut line: &LyricTimeLine = body.get(0).unwrap();
        for i in 1..l {
            let line_i = body.get(i).unwrap();
            duration = line_i.start - line.start;
            line = line_i;
            body[i - 1].duration.set(duration);
        }
    }

    pub fn get_next_lyric_by_time(&self, time: f64) -> LyricTimeLine {
        info!("get full time line by time: {}", time);
        let mut index = self.find_time_line_index(time);
        if index.is_none() {
            warn!("lyric time line not found");
            return Default::default();
        }

        let next_full_lrc = &self.lyric_body[index.unwrap()];
        info!("next {}", next_full_lrc.line.text.len());
        next_full_lrc.clone()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LyricLine {
    pub text: String,
    pub length: i64,
    pub word_timeline: Vec<WordTimeline>,
    pub translation: String,
    pub pronunciation: String,
}

impl LyricLine {
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    pub fn set_translation(&mut self, translation: String) {
        self.translation = translation;
    }
    pub fn set_pronunciation(&mut self, pronunciation: String) {
        self.pronunciation = pronunciation;
    }
    pub fn set_word_timeline(&mut self, word_timeline: &Vec<WordTimeline>) {
        self.word_timeline = word_timeline.to_vec();
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WordTimeline {
    pub start: f64,
    pub duration: f64,
    pub pos: usize, // 在整个字符串中的位置,当前字轴位置应包含[0, pos)
}

pub trait LyricLineResponse {
    fn get_lyric_line(&self) -> LyricLine;
}

impl LyricLine {
    pub fn new(
        text: String,
        length: i64,
        word_timeline: Vec<WordTimeline>,
        translation: String,
        pronunciation: String,
    ) -> LyricLine {
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
    pub fn new(pos: usize, start: f64, duration: f64) -> WordTimeline {
        WordTimeline {
            start,
            duration,
            pos,
        }
    }
}
