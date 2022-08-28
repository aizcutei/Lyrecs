use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct Song {
	pub title: String,
    pub artist: String,
    pub album: String,
}

pub trait RemoteSongTrait { // TODO: 换个好点的名字
    fn new(song: &Value) -> Self;
    fn new_empty() -> Self;
    fn is_empty(&self) -> bool;
}
