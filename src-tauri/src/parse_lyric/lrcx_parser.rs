use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

use regex::Regex;
use anyhow::Result as AnyResult;

use crate::parse_lyric::parser;

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
        let timestamp = parser::time_f64_to_time_tag(self.timestamp);
        write!(f, "[{}]{}", timestamp, self.verse)
    }
}

impl Lrcx {
    pub fn new() -> Lrcx {
        Lrcx {
            metadata: BTreeSet::new(),
            lyric_body: Vec::new(),
        }
    }

    pub fn from_str(s: String) -> AnyResult<Lrcx> {
        let mut lrcx = Lrcx::new();
        let lines: Vec<&str> = s.split('\n').collect();
        let LRC_METATAG_REGEX = Regex::new(r#"\[[a-z]+\]"#).unwrap();
        let LRC_TIMELINE_REGEX = Regex::new(r#"\[.*\]:.*"#).unwrap();

        for line in lines {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if LRC_METATAG_REGEX.captures(line).is_some() {
                let re = LRC_METATAG_REGEX.captures(line).unwrap();
                let tag_name = re.get(0).unwrap().as_str();
                let tag_value = line[tag_name.len() + 1..].trim().to_string();
                lrcx.metadata.insert(IDTag::new(tag_name.to_string(), tag_value));
                continue;
            }

            if LRC_TIMELINE_REGEX.captures(line).is_some() {
                let re = LRC_TIMELINE_REGEX.captures(line).unwrap();
                let timestamp = re.get(0).unwrap().as_str();
                let timestamp = parser::time_tag_to_time_f64(timestamp);
                let verse = line[timestamp.to_string().len() + 2..].trim().to_string();
                lrcx.lyric_body.push(LyricInline::new(timestamp, verse));
                continue;
            }
        }
        Ok(lrcx)
    }

    pub fn add_line(&mut self, line: String) -> AnyResult<Lrcx> {
        let LRC_TIMELINE_REGEX = Regex::new(r#"\[.*\]:.*"#).unwrap();
        if LRC_TIMELINE_REGEX.captures(&line).is_some() {
            let re = LRC_TIMELINE_REGEX.captures(&line).unwrap();
            let timestamp = re.get(0).unwrap().as_str();
            let timestamp = parser::time_tag_to_time_f64(timestamp);
            let verse = line[timestamp.to_string().len() + 2..].trim().to_string();
            self.lyric_body.push(LyricInline::new(timestamp, verse));
            Ok(self.clone())
        }else{
            anyhow::private::Err(anyhow::Error::msg("Invalid line"))
        }
    }

    pub fn find_time_line_index(&self,time: f64) -> Option<usize> {
        for (i, line) in self.lyric_body.iter().enumerate().rev() {
            if line.timestamp <= time {
                return Some(i);
            }
        }
        None
    }

    pub fn get_time_line_by_index(&self,index: usize) -> Option<LyricInline> {
        self.lyric_body.get(index).cloned()
    }

    pub fn get_time_line_by_time(&self,time: f64) -> Option<LyricInline> {
        let index = self.find_time_line_index(time);
        if index.is_some() {
            self.lyric_body.get(index.unwrap()).cloned()
        }else{
            None
        }
    }

    pub fn iter(&self) -> std::slice::Iter<LyricInline> {
        self.lyric_body.iter()
    }
}

impl Display for Lrcx {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in self.lyric_body.iter() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}