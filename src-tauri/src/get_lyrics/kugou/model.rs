// ----- Kugou -----

use std::{cell::Cell, ops::Index};

use serde_json::Value;

use crate::{
    api::model::{IDTag, Lrcx, LyricTimeLine, WordTimeline},
    get_lyrics::song::RemoteSongTrait,
};

#[derive(Debug, Clone)]
pub struct KugouSong {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub hash: String,
    pub id: String,
    pub access_key: String,
}

#[derive(Debug, Clone)]
pub struct KugouSongList(Vec<KugouSong>);

impl RemoteSongTrait for KugouSong {
    fn new(song: &Value) -> Self {
        let mut name = "".to_string();
        if song.get("name").is_none() {
            if song.get("song").is_some() {
                name = song["song"].as_str().unwrap().to_string();
            }
        } else {
            name = song["name"].as_str().unwrap().to_string();
        }

        let mut artist = "".to_string();
        if song.get("singername").is_none() {
            if song.get("singer").is_some() {
                artist = song["singer"].as_str().unwrap().to_string();
            }
        } else {
            artist = song["singername"].as_str().unwrap().to_string();
        }

        let mut album = "".to_string();
        if song.get("album_name").is_some() {
            album = song["album_name"].as_str().unwrap().to_string();
        }

        let mut hash = "".to_string();
        if song.get("hash").is_some() {
            hash = song["hash"].as_str().unwrap().to_string();
        }

        let mut id = "".to_string();
        if song.get("id").is_some() {
            id = song["id"].as_str().unwrap().to_string();
        }

        let mut access_key = "".to_string();
        if song.get("accesskey").is_some() {
            access_key = song["accesskey"].as_str().unwrap().to_string();
        }

        KugouSong {
            hash,
            id,
            access_key,
            name,
            artist,
            album,
        }
    }

    fn new_empty() -> Self {
        KugouSong {
            hash: String::new(),
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            id: String::new(),
            access_key: String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_empty()
            && self.artist.is_empty()
            && self.album.is_empty()
            && self.hash.is_empty()
    }
}

impl KugouSongList {
    pub fn new() -> Self {
        KugouSongList(Vec::new())
    }
    pub fn push(&mut self, song: KugouSong) {
        self.0.push(song);
    }
}

#[derive(Debug, Clone)]
pub struct KugouSongLyrics {
    pub content: String,
    pub decoded: String,
}

impl KugouSongLyrics {
    pub fn new(content: String) -> Self {
        KugouSongLyrics {
            content,
            decoded: "".to_string(),
        }
    }

    pub fn new_empty() -> Self {
        KugouSongLyrics {
            content: String::new(),
            decoded: String::new(),
        }
    }

    pub fn to_lrcx(&self) -> Lrcx {
        let mut lrcx: Lrcx = Default::default();
        let lines = self.decoded.clone();
        let lines_iter = &lines
            .split('\n')
            .map(|x| x.replace('\r', ""))
            .collect::<Vec<String>>();
        let mut i = 0;
        // 处理前面的 metadata 部分
        for line in lines_iter {
            let tag = line.trim_start_matches('[').trim_end_matches(']');
            let tag_iter = tag.split(':').collect::<Vec<&str>>();
            i += 1; // 移到后面一行，这样 parse 到 language 之后break就直接从歌词开始
            if tag_iter[0] == "language" {
                break;
            }
            lrcx.metadata
                .insert(IDTag::new(tag_iter[0].to_string(), tag_iter[1].to_string()));
        }

        // 处理歌词部分
        for line in lines_iter[i..].iter() {
            if !line.starts_with('[') {
                continue;
            }
            let mut timeline: LyricTimeLine = Default::default();
            let bracket_split = line
                .trim_start_matches('[')
                .splitn(2, ']')
                .collect::<Vec<&str>>();
            // 一行歌词的时间轴
            let timestamp = bracket_split[0].split(',').collect::<Vec<&str>>();

            let start = timestamp[0].parse::<f64>();
            if start.is_err() {
                continue;
            }
            timeline.start = start.unwrap() / 1000.0;
            // 没有start就没有end，应该不会有没end有start，所以这里偷懒
            timeline.duration = Cell::new(timestamp[1].parse::<f64>().unwrap() / 1000.0);

            // 处理字轴

            // 如果没有字轴,直接开溜
            if bracket_split[1].find(['<', '>']).is_none() {
                timeline.line.text = bracket_split[1].to_string();
                lrcx.lyric_body.push(timeline);
                continue;
            }

            // todo 要是歌词里有<就会出问题
            let mut lyric_split = bracket_split[1].split('<').collect::<Vec<&str>>();
            if lyric_split.len() == 1 {}
            // println!("lyric_split: {:?}", lyric_split);

            let mut text = String::new();
            let mut word_pos: usize = 0;
            for raw_word in lyric_split {
                // 先处理时间
                if raw_word.find('>').is_none() {
                    continue;
                }
                let word_time = &raw_word[..raw_word.find('>').unwrap()];
                let mut word_time_line: WordTimeline = Default::default();
                let word_time_split = word_time.split(',').collect::<Vec<&str>>();
                word_time_line.start =
                    timeline.start + (word_time_split[0].parse::<f64>().unwrap() / 1000.0);
                word_time_line.duration = word_time_split[1].parse::<f64>().unwrap() / 1000.0;

                // 再处理文本
                let word = &raw_word[raw_word.find('>').unwrap() + 1..];
                word_time_line.pos = {
                    word_pos += word.chars().count();
                    word_pos
                };
                text.push_str(word);
                timeline.line.word_timeline.push(word_time_line);
            }
            timeline.line.text = text;
            lrcx.lyric_body.push(timeline);
        }
        lrcx
    }
}

impl Index<usize> for KugouSongList {
    type Output = KugouSong;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::iter::Iterator for KugouSongList {
    type Item = KugouSong;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl ExactSizeIterator for KugouSongList {
    fn len(&self) -> usize {
        self.0.len()
    }
}
