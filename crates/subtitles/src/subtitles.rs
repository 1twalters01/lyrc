use crate::{
    formats::lrc::{parser::LrcParser, writer::LrcWriter},
    language::Language,
    parser::SubtitleParser,
    writer::SubtitleWriter,
};
use chrono::Duration;
use std::{fs, path::PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum SyncLevel {
    None,
    Cue,
    Word,
}

#[derive(Clone, Debug)]
pub struct SubtitleDocument {
    pub metadata: SubtitleMetadata,
    pub cues: Vec<SubtitleCue>,
}

impl SubtitleDocument {
    pub fn sync_level(&self) -> SyncLevel {
        if self
            .cues
            .iter()
            .any(|cue| matches!(cue.content, SubtitleContent::Words(_)))
        {
            SyncLevel::Word
        } else if self.cues.iter().any(|cue| {
            // cue.start
            true
        }) {
            SyncLevel::Cue
        } else {
            SyncLevel::None
        }
    }

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
        subtitle_document: &SubtitleDocument,
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

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.metadata.file_path {
            Some(file_path) => {
                let file = SubtitleDocument::write(self)?;
                Ok(fs::write(file_path, file)?)
            }
            None => Ok(()),
        }
    }

    pub fn update_languages(&mut self) {
        let text = self
            .cues
            .iter()
            .filter_map(|cue| match &cue.content {
                SubtitleContent::Text(text) => Some(text.as_str()),
                SubtitleContent::Words(_) => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        if let Some(info) = whatlang::detect(&text) {
            self.metadata.languages.push(info.lang().into());
        }

        self.metadata.languages.dedup();
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
    pub languages: Vec<Language>,
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
    pub id: Uuid,
    pub start: Duration,
    pub end: Duration,
    pub content: SubtitleContent,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubtitleContent {
    Text(String),
    Words(Vec<AlignedWord>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlignedWord {
    pub start: Duration,
    pub end: Duration,
    pub content: String,
}
