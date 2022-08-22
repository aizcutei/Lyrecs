// ----- Kugou -----

use std::ops::Index;

use serde_json::Value;

use crate::get_lyrics::song::Song;

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
pub struct KugouSongList (Vec<KugouSong>);

impl Song for KugouSong {
    fn new(song: &Value) -> Self {
        let mut name = "".to_string();
        if song.get("name").is_none() {
            if song.get("song").is_some(){
                name = song["song"].as_str().unwrap().to_string();
            }
        } else {
            name = song["name"].as_str().unwrap().to_string();
        }

        let mut artist = "".to_string();
        if song.get("singername").is_none() {
            if song.get("singer").is_some(){
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

        KugouSong{
            hash,
            id,
            access_key,
            name,
            artist,
            album,
        }
    }

    fn new_empty() -> Self {
        KugouSong{
            hash: String::new(),
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            id: String::new(),
            access_key: String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.artist.is_empty() && self.album.is_empty() && self.hash.is_empty()
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
    pub fn new(lyric: &Value) -> Self {
        let mut content = "".to_string();
        if lyric.get("content").is_some() {
            content = lyric["content"].as_str().unwrap().to_string();
        }
        KugouSongLyrics{
            content,
            decoded: "".to_string(),
        }
    }

    pub fn new_empty() -> Self {
        KugouSongLyrics{
            content: String::new(),
            decoded: String::new(),
        }
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
