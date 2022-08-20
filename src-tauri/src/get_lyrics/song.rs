use serde_json::Value;

pub trait Song {
    fn new(song: &Value) -> Self;
    fn new_empty() -> Self;
    fn is_empty(&self) -> bool;
}
