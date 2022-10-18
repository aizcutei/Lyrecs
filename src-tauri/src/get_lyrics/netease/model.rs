use std::ops::Index;

use serde_json::Value;

use crate::get_lyrics::song::RemoteSongTrait;

//----- Netease ------
#[derive(Debug, Default, Clone)]
pub struct NeteaseSong {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub id: String,
}

#[derive(Default, Debug, Clone)]
pub struct NeteaseSongList(pub Vec<NeteaseSong>);

#[derive(Debug, Clone)]
pub struct NeteaseSongLyrics {
    pub lyric: String,
    pub tlyric: String,
    pub klyric: String,
}

impl ExactSizeIterator for NeteaseSongList {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl NeteaseSongLyrics {
    pub fn new() -> Self {
        NeteaseSongLyrics {
            lyric: String::new(),
            tlyric: String::new(),
            klyric: String::new(),
        }
    }

    pub fn get_original_lyric(&self) -> Option<String> {
        Some(self.lyric.clone())
    }

    pub fn get_translated_lyric(&self) -> Option<String> {
        Some(self.tlyric.clone())
    }
}

impl NeteaseSongList {
    pub fn new() -> Self {
        NeteaseSongList(Vec::new())
    }
    pub fn push(&mut self, song: NeteaseSong) {
        self.0.push(song);
    }
}

impl RemoteSongTrait for NeteaseSong {
    fn new(song: &Value) -> Self {
        NeteaseSong {
            name: song["name"].as_str().unwrap().to_string(),
            artist: song["artists"][0]["name"].as_str().unwrap().to_string(),
            album: song["album"]["name"].as_str().unwrap().to_string(),
            id: song["id"].as_i64().unwrap().to_string(),
        }
    }
    fn new_empty() -> Self {
        NeteaseSong {
            name: String::new(),
            artist: String::new(),
            album: String::new(),
            id: String::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.name.is_empty()
            && self.artist.is_empty()
            && self.album.is_empty()
            && self.id.is_empty()
    }
}

impl ToString for NeteaseSongLyrics {
    fn to_string(&self) -> String {
        let mut lyric = self.lyric.clone();
        let mut tlyric = self.tlyric.clone();
        let mut klyric = self.klyric.clone();
        if lyric.is_empty() {
            lyric = "[00:00.00] This Music No Lyric".to_string();
        }
        if tlyric.is_empty() {
            tlyric = "".to_string();
        }
        if klyric.is_empty() {
            klyric = "".to_string();
        }
        format!("{}\n{}\n{}", lyric, tlyric, klyric)
    }
}

impl Index<usize> for NeteaseSongList {
    type Output = NeteaseSong;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::iter::Iterator for NeteaseSongList {
    type Item = NeteaseSong;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl std::fmt::Display for NeteaseSong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nArtist: {}\nAlbum: {}\nID: {}\n\n",
            self.name, self.artist, self.album, self.id
        )
    }
}
