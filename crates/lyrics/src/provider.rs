use crate::models::Lyrics;
use futures::future::BoxFuture;
use mpris::track::Track;
use pyo3::PyErr;

use std::{error::Error, fmt};

pub trait LyricsProvider: Send + Sync {
    fn search(&self, track: Track) -> BoxFuture<'_, Result<Option<Lyrics>, LyricsError>>;
}

// use thiserror?
#[derive(Debug)]
pub enum LyricsError {
    PythonError { error: PyErr },
}

impl fmt::Display for LyricsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LyricsError::PythonError { error } => {
                write!(f, "Python error: {error}")
            }
        }
    }
}

impl Error for LyricsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LyricsError::PythonError { error } => Some(error),
        }
    }
}
