use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};

use log::info;
use regex::Regex;
use anyhow::Result as AnyResult;

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

impl Lrcx {
    pub fn new() -> Lrcx {
        Lrcx {
            metadata: BTreeSet::new(),
            lyric_body: Vec::new(),
        }
    }

    pub fn from_str(s: String, splitter: &str) -> AnyResult<Lrcx> {
        let mut lrcx = Lrcx::new();
        let mut lrc_string: &str = &s;

        if s.starts_with('"') {
            lrc_string = &s[1..s.len() - 1];
        }

        let lines: Vec<&str> = lrc_string.split(splitter).collect();
        info!("{} lines", lines.len());
        let lrc_metatag_regex = Regex::new(r#"\[[a-z]+\]"#).unwrap();
        let lrc_timeline_regex = Regex::new(r#"\[[0-9]+:[0-9]+.[0-9]+\]"#).unwrap();

        for line in lines {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if lrc_metatag_regex.captures(line).is_some() {
                let re = lrc_metatag_regex.captures(line).unwrap();
                let tag_name = re.get(0).unwrap().as_str();
                let tag_value = line[tag_name.len() + 1..].trim().to_string();
                lrcx.metadata.insert(IDTag::new(tag_name.to_string(), tag_value));
                continue;
            }
            if lrc_timeline_regex.captures(line).is_some() {
                let re = lrc_timeline_regex.captures(line).unwrap();
                let timestamp = re.get(0).unwrap().as_str();
                let verse = line[timestamp.to_string().len()..].trim().to_string();
                let timestamp = utils::time_tag_to_time_f64(timestamp[1..timestamp.len() - 1].trim());
                lrcx.lyric_body.push(LyricInline::new(timestamp, verse));
                continue;
            }
        }
        Ok(lrcx)
    }

    pub fn add_line(&mut self, line: String) -> AnyResult<Lrcx> {
        let lrc_timeline_regex = Regex::new(r#"\[[0-9]+:[0-9]+.[0-9]+\]"#).unwrap();
        if lrc_timeline_regex.captures(&line).is_some() {
            let re = lrc_timeline_regex.captures(&line).unwrap();
            let timestamp = re.get(0).unwrap().as_str();
            let verse = line[timestamp.to_string().len()..].trim().to_string();
            let timestamp = utils::time_tag_to_time_f64(timestamp[1..timestamp.len() - 1].trim());
            self.lyric_body.push(LyricInline::new(timestamp, verse));
            Ok(self.clone())
        }else{
            anyhow::private::Err(anyhow::Error::msg("Invalid line"))
        }
    }

    pub fn find_time_line_index(&self,time: f64) -> Option<usize> {
        for (i, line) in self.lyric_body.iter().enumerate().rev() {
            if line.timestamp <= time {
                return Some(i)
            }
        }
        None
    }

    pub fn get_time_line_by_index(&self,index: usize) -> Option<LyricInline> {
        self.lyric_body.get(index).cloned()
    }

    pub fn get_time_line_by_time(&self,time: f64) -> Option<LyricInline> {
        info!("get_time_line_by_time: {}", time);
        let index = self.find_time_line_index(time);
        if let Some(i) = index {
            info!("getting lyric index {}", index.unwrap());
            return self.lyric_body.get(i).cloned()
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<LyricInline> {
        self.lyric_body.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.lyric_body.is_empty()
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