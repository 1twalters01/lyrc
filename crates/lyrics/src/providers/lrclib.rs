use futures::future::BoxFuture;
use mpris::track::Track;
use pyo3::{prelude::*, types::PyDict};
use pyo3_async_runtimes::tokio::into_future;

use crate::{
    models::{Lyrics, LyricsFormat, LyricsSource},
    provider::{LyricsError, LyricsProvider},
};

pub struct LrclibProvider;

impl LyricsProvider for LrclibProvider {
    fn search(&self, track: Track) -> BoxFuture<'static, Result<Option<Lyrics>, LyricsError>> {
        Box::pin(async move {
            let result = Python::attach(|py| {
                // import required python packages
                let httpx = PyModule::import(py, "httpx")?;
                let datetime = PyModule::import(py, "datetime")?;

                // import modules
                let service_module = PyModule::import(py, "scraper.service")?;
                let provider_module = PyModule::import(py, "scraper.providers.lrclib")?;
                let track_module = PyModule::import(py, "scraper.models.track")?;

                // get provider dict with lrclib instance inside
                let client = httpx.getattr("AsyncClient")?.call0()?;
                let lrclib_provider = provider_module
                    .getattr("LrcLibProvider")?
                    .call1((client,))?;
                let providers = PyDict::new(py);
                providers.set_item("lrclib", lrclib_provider)?;

                // Create instance of lyrics service
                let lyrics_service = service_module
                    .getattr("LyricsService")?
                    .call1((providers,))?;

                // get python track class
                let timedelta = datetime.getattr("timedelta")?.call1((
                    track.duration.num_days(),
                    track.duration.num_seconds(),
                    track.duration.num_microseconds().unwrap_or(0),
                ))?;

                let py_track = track_module.getattr("Track")?.call1((
                    track.title,
                    track.artists.first(),
                    track.album,
                    timedelta,
                ))?;

                // run service.search(track, "lrclib")
                let coroutine = lyrics_service.call_method1("search", (py_track, "lrclib"))?;
                into_future(coroutine)
            })
            .map_err(|e| LyricsError::PythonError { error: e })?;

            let py_result = result
                .await
                .map_err(|e| LyricsError::PythonError { error: e })?;

            let lyrics: Option<Lyrics> = Python::attach(|py| {
                if py_result.is_none(py) {
                    return Ok(None);
                }

                let lyrics = py_result.bind(py);
                let content: String = lyrics.getattr("content")?.extract()?;
                let py_format: String = lyrics.getattr("format")?.getattr("value")?.extract()?;
                let py_source: String = lyrics.getattr("source")?.getattr("value")?.extract()?;

                let format = match py_format.as_str() {
                    "lrc" => LyricsFormat::Lrc,
                    "text" => LyricsFormat::Text,
                    _ => {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "unknown lyrics format",
                        ));
                    }
                };
                let source = match py_source.as_str() {
                    "lrclib" => LyricsSource::Lrclib,
                    _ => {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "unknown lyrics source",
                        ));
                    }
                };

                Ok(Some(Lyrics {
                    content,
                    format,
                    source,
                }))
            })
            .map_err(|e| LyricsError::PythonError { error: e })?;

            Ok(lyrics)
        })
    }
}
