use serde_json::Value;
use std::ops::Index;
pub struct Song {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub id: String,
    
}

pub struct SongList (Vec<Song>);

pub struct SongLyrics {
    pub lyric: String,
    pub tlyric: String,
    pub klyric: String,
}

impl SongLyrics {
    pub fn new() -> SongLyrics {
        SongLyrics {
            lyric: String::new(),
            tlyric: String::new(),
            klyric: String::new(),
        }
    }
}

impl SongList {
    pub fn new() -> Self {
        SongList(Vec::new())
    }
    pub fn push(&mut self, song: Song) {
        self.0.push(song);
    }
}

impl Song {
    pub fn new(song: &Value) -> Song {
        Song {
            name: song["name"].as_str().unwrap().to_string(),
            artist: song["artists"][0]["name"].as_str().unwrap().to_string(),
            album: song["album"]["name"].as_str().unwrap().to_string(),
            id: song["id"].as_i64().unwrap().to_string(),
        }
    }
}

impl ToString for SongLyrics {
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

impl Index<usize> for SongList {
    type Output = Song;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::iter::Iterator for SongList {
    type Item = Song;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl std::fmt::Debug for SongList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SongList(")?;
        for song in &self.0 {
            write!(f, "{:?}", song)?;
        }
        write!(f, ")")
    }
}

impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\nArtist: {}\nAlbum: {}\nID: {}\n\n", self.name, self.artist, self.album, self.id)
    }
}

impl std::fmt::Debug for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\nArtist: {}\nAlbum: {}\nID: {}\n\n", self.name, self.artist, self.album, self.id)
    }
}

impl Clone for Song {
    fn clone(&self) -> Self {
        Song {
            name: self.name.clone(),
            artist: self.artist.clone(),
            album: self.album.clone(),
            id: self.id.clone(),
        }
    }
}
