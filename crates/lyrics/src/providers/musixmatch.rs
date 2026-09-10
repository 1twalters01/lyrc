use futures::future::BoxFuture;
use mpris::track::Track;
use pyo3::{prelude::*, types::PyDict};
use pyo3_async_runtimes::tokio::into_future;

use crate::{
    error::LyricsError,
    models::{Lyrics, LyricsFormat, LyricsSource},
    provider::LyricsDownloader,
};

pub struct MusixmatchProvider;

impl LyricsDownloader for MusixmatchProvider {
    fn search(&self, track: Track) -> BoxFuture<'static, Result<Option<Lyrics>, LyricsError>> {
    }
}
