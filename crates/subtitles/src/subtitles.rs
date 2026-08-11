use crate::{
    formats::lrc::{parser::LrcParser, writer::LrcWriter},
    parser::SubtitleParser,
    writer::SubtitleWriter,
};
use chrono::Duration;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct SubtitleDocument {
    pub metadata: SubtitleMetadata,
    pub cues: Vec<SubtitleCue>,
}

impl SubtitleDocument {
    pub fn from_pathbuf(path: PathBuf) -> Result<SubtitleDocument, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        match &path.extension() {
            // Make this an actual error type
            None => Err(String::from("File does not have an extension").into()),
            Some(os_str) => match os_str.to_str() {
                Some("lrc") => {
                    let lrc_parser = LrcParser;
                    let mut subtitle_document = lrc_parser.parse(&content)?;
                    subtitle_document.metadata.file_path = Some(path);
                    Ok(subtitle_document)
                }
                Some(_) => Err(String::from("unknown file type").into()),
                None => Err(String::from("os str cannot be turned into a &str").into()),
            },
        }
    }

    pub fn write(
        subtitle_document: SubtitleDocument,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match &subtitle_document.metadata.file_path {
            Some(file_path) => match file_path.extension() {
                Some(os_str) => match os_str.to_str() {
                    Some("lrc") => {
                        let lrc_writer = LrcWriter;
                        let lrc_file = lrc_writer.write(&subtitle_document.clone())?;
                        Ok(lrc_file)
                    }
                    Some(_) => Err(String::from("unknown file type").into()),
                    None => Err(String::from("os str cannot be turned into a &str").into()),
                },
                None => Err(String::from("File does not have an extension").into()),
            },
            None => Err(String::from("File path does not exist").into()),
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
    pub languages: Vec<String>,
    pub file_path: Option<PathBuf>,
}

impl Default for SubtitleMetadata {
    fn default() -> Self {
        Self {
            album: None,
            title: None,
            artists: Vec::new(),
            languages: Vec::new(),
            file_path: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubtitleCue {
    pub id: Option<String>,
    pub start: Duration,
    pub end: Option<Duration>,
    pub content: SubtitleContent,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubtitleContent {
    Text(String),
}
