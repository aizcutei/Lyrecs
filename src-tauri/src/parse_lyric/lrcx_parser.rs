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
        let mut index: usize = 0;
        for (i, line) in self.lyric_body.iter().enumerate().rev() {
            if line.timestamp <= time {
                index = i;
            }
        }
        while index > 0 &&
            self.get_first_time_line_by_index(index).unwrap().timestamp == self.get_first_time_line_by_index(index - 1).unwrap().timestamp {
                index -= 1;
        }
        Some(index)
    }

    pub fn get_first_time_line_by_index(&self,index: usize) -> Option<LyricInline> {
        info!("getting first line lyric index {}", index);
        self.lyric_body.get(index).cloned()
    }

    pub fn get_full_time_line_by_index(&self, i: usize) -> Vec<&LyricInline> {
        info!("get full time line by index: {}", i);
        let mut index = i;
        let mut full_time_line = vec![self.lyric_body.get(index).unwrap()];
        info!(" - Find one time line by index: {}", i);

        while self.get_first_time_line_by_index(index).unwrap().timestamp == self.get_first_time_line_by_index(index + 1).unwrap().timestamp {
            full_time_line.push(self.lyric_body.get(index + 1).unwrap());
            info!(" - Find one time line by index: {}", index + 1);
            index += 1;
        }
        full_time_line
    }

    pub fn get_first_time_line_by_time(&self, time: f64) -> Option<LyricInline> {
        info!("get first time line by time: {}", time);
        let index = self.find_time_line_index(time);
        if let Some(i) = index {
            return self.lyric_body.get(i).cloned()
        }
        None
    }

    pub fn get_next_lyric_by_time(&self, time:f64) -> LyricLine {
        info!("get full time line by time: {}", time);
        let mut index = self.find_time_line_index(time);

        if index.is_none() {
            return Default::default();
        }

        let mut result : LyricLine = Default::default();
        let next_full_lrc = self.get_full_time_line_by_index(index.unwrap());
        info!("next {}", next_full_lrc.len());
        if next_full_lrc.len() == 0{
            result.length = -1;
        }else if next_full_lrc.len() == 1 {
            result.text = next_full_lrc.get(0).unwrap().lyric_str();
        }else {
            for line in next_full_lrc {
                if line.lyric_str().starts_with("[tt]"){
                    let key_frame_str = &line.to_string()[4..];
                    let key_frame = key_frame_str.split(' ');
                    unimplemented!()
                }
            }
        }
        result
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
