use chrono::Duration;

#[derive(Clone)]
pub struct SubtitleDocument {
    pub metadata: SubtitleMetadata,
    pub cues: Vec<SubtitleCue>,
}

impl Default for SubtitleDocument {
    fn default() -> Self {
        let metadata = SubtitleMetadata::default();
        let cues = Vec::new();

        Self { metadata, cues }
    }
}

#[derive(Clone)]
pub struct SubtitleMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub language: Option<String>,
}

impl Default for SubtitleMetadata {
    fn default() -> Self {
        Self {
            title: None,
            artist: None,
            album: None,
            language: None,
        }
    }
}

#[derive(Clone)]
pub struct SubtitleCue {
    pub id: Option<String>,
    pub start: Duration,
    pub end: Option<Duration>,
    pub content: SubtitleContent,
}

#[derive(Clone)]
pub enum SubtitleContent {
    Text(String),
}
