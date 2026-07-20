pub struct Lyrics {
    pub content: String,
    pub format: LyricsFormat,
    pub source: LyricsSource,
}

pub enum LyricsFormat {
    Lrc,
    Text,
}

pub enum LyricsSource {
    Lrclib,
}
