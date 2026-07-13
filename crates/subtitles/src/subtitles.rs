use chrono::Duration;

pub struct SubtitleDocument {
    pub metadata: SubtitleMetadata,
    pub cues: Vec<SubtitleCue>,
}

pub struct SubtitleMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub language: Option<String>,
}

pub struct SubtitleCue {
    pub id: Option<String>,
    pub start: Duration,
    pub end: Option<Duration>,
    pub text: String,
}
