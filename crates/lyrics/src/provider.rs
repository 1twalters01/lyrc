use crate::models::Lyrics;
use futures::future::BoxFuture;
use mpris::track::Track;
use pyo3::PyErr;

pub trait LyricsProvider: Send + Sync {
    fn search(&self, track: Track) -> BoxFuture<'_, Result<Option<Lyrics>, LyricsError>>;
}

pub enum LyricsError {
    PythonError { error: PyErr },
}
