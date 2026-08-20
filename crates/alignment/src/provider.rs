use std::{error::Error, fmt, path::PathBuf};

use pyo3::PyErr;
use subtitles::subtitles::SubtitleDocument;

pub trait LyricsAligner: Send + Sync {
    fn align_cues(
        // &self,
        audio_file_path: PathBuf,
        subtitle_document: SubtitleDocument,
    ) -> Result<Option<SubtitleDocument>, AlignmentError>;
}

// use thiserror?
#[derive(Debug)]
pub enum AlignmentError {
    PythonError { error: PyErr },
    NoAudioFilePath,
    InvalidAudioPath,
    NoLanguageCode,
    AlreadyAligned,
}

impl From<PyErr> for AlignmentError {
    fn from(error: PyErr) -> Self {
        AlignmentError::PythonError { error }
    }
}

impl fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignmentError::PythonError { error } => {
                write!(f, "Python error: {error}")
            }
            AlignmentError::NoAudioFilePath => {
                write!(f, "No audio file path found for current song")
            }
            AlignmentError::InvalidAudioPath => {
                write!(f, "Invalid audio path")
            }
            AlignmentError::NoLanguageCode => {
                write!(f, "No language code found")
            }
            AlignmentError::AlreadyAligned => {
                write!(f, "Content is already aligned")
            }
        }
    }
}

impl Error for AlignmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AlignmentError::PythonError { error } => Some(error),
            AlignmentError::NoAudioFilePath => None,
            AlignmentError::InvalidAudioPath => None,
            AlignmentError::NoLanguageCode => None,
            AlignmentError::AlreadyAligned => None,
        }
    }
}
