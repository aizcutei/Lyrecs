use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};

use log::info;
use regex::Regex;
use anyhow::Result as AnyResult;

use crate::api::model::LyricLine;
use crate::parse_lyric::utils;

#[derive(Debug, Clone)]
pub struct LyricInline {
    timestamp: f64,
    verse: String,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
pub struct IDTag {
    tag_name: String,
    tag_value: String,
}

#[derive(Debug, Clone)]
pub struct Lrcx {
    pub metadata: BTreeSet<IDTag>,
    lyric_body: Vec<LyricInline>,
}

impl LyricInline {
    pub fn new(timestamp: f64, verse: String) -> LyricInline {
        LyricInline {
            timestamp,
            verse,
        }
    }
    pub fn lyric_str(&self) -> String {
        self.verse.clone()
    }

}

impl IDTag {
    fn new(tag_name: String, tag_value: String) -> IDTag {
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

impl Display for LyricInline {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let timestamp = utils::time_f64_to_time_tag(self.timestamp);
        write!(f, "[{}]{}", timestamp, self.verse)
    }
}
