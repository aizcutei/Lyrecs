use std::collections::BTreeSet;

use regex::Regex;
use anyhow::Result as AnyResult;

struct LyricInline {
    timestamp: f64,
    verse: String,
}

struct IDTag {
    tag_name: String,
    tag_value: String,
}

struct Lrcx {
    pub metadata: BTreeSet<IDTag>,
    lyric_body: Vec<LyricInline>,
}

impl LyricInline {
    fn new(timestamp: f64, verse: String) -> LyricInline {
        LyricInline {
            timestamp,
            verse,
        }
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
                
            }


        }

        Ok(lrcx)
    }
}