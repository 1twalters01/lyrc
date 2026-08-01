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
    pub album: Option<String>,
    pub title: Option<String>,
    pub artists: Vec<String>,
    pub language: Option<String>,
    pub file_path: Option<PathBuf>,
}

impl Default for SubtitleMetadata {
    fn default() -> Self {
        Self {
            album: None,
            title: None,
            artists: Vec::new(),
            language: None,
            file_path: None,
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
