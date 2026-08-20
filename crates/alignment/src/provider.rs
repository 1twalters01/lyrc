use std::{error::Error, fmt};

use futures::future::BoxFuture;
use mpris::track::Track;
use subtitles::subtitles::SubtitleDocument;
use pyo3::PyErr;

pub trait LyricsAligner: Send + Sync {
    fn align_cues(&self, track: Track, subtitle_document: SubtitleDocument) -> BoxFuture<'_, Result<Option<SubtitleDocument>, AlignmentError>>;
}

// use thiserror?
#[derive(Debug)]
pub enum AlignmentError {
    PythonError { error: PyErr },
}

impl fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignmentError::PythonError { error } => {
                write!(f, "Python error: {error}")
            }
        }
    }
}

impl Error for AlignmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AlignmentError::PythonError { error } => Some(error),
        }
    }
}
