use regex;
use anyhow::Result as AnyResult;

struct LyricInline {
    timestamp: f64,
    verse: String,
}

struct IDTag {
    id: String,
}

struct Lrcx {
    pub metadata: BTreeSet<IDtag>,
    lyric_body: Vec<LyricInline>,
}

impl Lrcx {
    pub fn new() -> Lrcx {
        Lrcx {
            metadata: BTreeSet::new(),
            lyric_body: Vec::new(),
        }
    }

    pub fn from_str() -> AnyResult<Lrcx> {
        let lrcx = Lrcx::new();
        Ok(lrcx)
    }
}