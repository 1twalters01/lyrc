use chrono::Duration;
use crate::parser::Parser;
use std::{
    ffi::OsStr,
    path::PathBuf
};

#[derive(Clone)]
pub struct SubtitleDocument {
    pub metadata: SubtitleMetadata,
    pub cues: Vec<SubtitleCue>,
}

impl SubtitleDocument {
    pub fn from_pathbuf(path: PathBuf) -> Result<SubtitleDocument, <P as Parser>::Error> {
        let extension: Option<OsStr> = path.extension();
        match extension {
            Some(extension) => match extension {
                "lrc" => {
                    // read file
                    // pass to lrc parser
                }
            },
            None => return Err() // How to express this error?
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
