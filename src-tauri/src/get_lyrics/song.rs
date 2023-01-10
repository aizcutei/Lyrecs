use serde_json::Value;

use crate::api::model::Lrcx;
use anyhow::Result as AnyResult;
#[derive(Debug, Clone, Default)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub trait RemoteSongTrait {
    // TODO: 换个好点的名字
    fn new(song: &Value) -> Self;
    fn new_empty() -> Self;
    fn is_empty(&self) -> bool;
}

pub trait Parsable {
    fn parse(&self) -> AnyResult<Lrcx>;
}
