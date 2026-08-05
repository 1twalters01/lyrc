use crate::{
    formats::lrc::parser::{LrcError, LrcParser},
    parser::SubtitleParser,
};
use chrono::Duration;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct SubtitleDocument {
    pub metadata: SubtitleMetadata,
    pub cues: Vec<SubtitleCue>,
}

impl SubtitleDocument {
    pub fn from_pathbuf(path: PathBuf) -> Result<SubtitleDocument, LrcError> {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => return Err(LrcError::InvalidMetadata),
        };
        match &path.extension() {
            None => panic!("error"),
            Some(os_str) => match os_str.to_str() {
                Some("lrc") => {
                    let lrc_parser = LrcParser;
                    lrc_parser.parse(&content)
                }
                _ => panic!("error"),
            },
        }
    }
}

impl Default for SubtitleDocument {
    fn default() -> Self {
        let metadata = SubtitleMetadata::default();
        let cues = Vec::new();

        Self { metadata, cues }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct SubtitleCue {
    pub id: Option<String>,
    pub start: Duration,
    pub end: Option<Duration>,
    pub content: SubtitleContent,
}

#[derive(Clone, Debug)]
pub enum SubtitleContent {
    Text(String),
}
