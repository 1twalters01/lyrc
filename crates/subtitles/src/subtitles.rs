use chrono::Duration;

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

pub struct SubtitleCue {
    pub id: Option<String>,
    pub start: Duration,
    pub end: Option<Duration>,
    pub text: String,
}
