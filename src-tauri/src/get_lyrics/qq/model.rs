use std::ops::Index;

use serde_json::Value;

use crate::get_lyrics::song::RemoteSongTrait;

//-----qq-----
#[derive(Default, Debug, Clone)]
pub struct QQSong {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub id: String,
    pub mid: String,
}

#[derive(Default, Debug, Clone)]
pub struct QQSongList (Vec<QQSong>);

#[derive(Debug, Clone)]
pub struct QQSongLyrics {
    pub lyric: String,
}

impl ExactSizeIterator for QQSongList {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl QQSongLyrics {
    pub fn new(json: &Value) -> Self {
        QQSongLyrics {
            lyric: json["lyric"].as_str().unwrap().to_string(),
        }
    }

    pub fn get_lyric(&self) -> Option<String> {
        Some(self.lyric.clone())
    }
}

impl QQSongList {
    pub fn new() -> Self {
        QQSongList(Vec::new())
    }
    pub fn push(&mut self, song: QQSong) {
        self.0.push(song);
    }
}

impl RemoteSongTrait for QQSong {
    fn new(song: &Value) -> Self {
        QQSong {
            name: song["name"].as_str().unwrap().to_string(),
            artist: song["singer"][0]["name"].as_str().unwrap().to_string(),
            album: song["album"]["name"].as_str().unwrap().to_string(),
            id: song["id"].as_i64().unwrap().to_string(),
            mid: song["mid"].as_str().unwrap().to_string(),
        }
    }

    fn new_empty() -> Self {
        QQSong {
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            id: String::new(),
            mid: String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.artist.is_empty() && self.album.is_empty() && self.id.is_empty() && self.mid.is_empty()
    }
}


impl Index<usize> for QQSongList {
    type Output = QQSong;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::iter::Iterator for QQSongList {
    type Item = QQSong;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl std::fmt::Display for QQSong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.artist)
    }
}
