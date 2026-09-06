use std::path::PathBuf;

use chrono::Duration;
use pyo3::{prelude::*, types::PyList};
use subtitles::{
    language::Language,
    subtitles::{SubtitleContent, SubtitleCue, SubtitleDocument, SubtitleMetadata},
};

use crate::error::TranslationError;

pub fn timedelta_to_duration(timedelta: &Bound<'_, PyAny>) -> PyResult<Duration> {
    let days: i64 = timedelta.getattr("days")?.extract()?;
    let seconds: i64 = timedelta.getattr("seconds")?.extract()?;
    let microseconds: i64 = timedelta.getattr("microseconds")?.extract()?;

    Ok(chrono::Duration::days(days)
        + chrono::Duration::seconds(seconds)
        + chrono::Duration::microseconds(microseconds))
}

pub fn convert_py_cues_to_translated_subtitle_document(
    py_translated_cues: Py<PyAny>,
    subtitle_document: SubtitleDocument,
    language: Language,
) -> Result<Option<SubtitleDocument>, TranslationError> {
    let translated_cues = convert_py_cues_to_cues(py_translated_cues, &subtitle_document)?;

    let translated_metadata = generate_translated_metadata(subtitle_document, language);

    Ok(Some(SubtitleDocument {
        metadata: translated_metadata,
        cues: translated_cues,
    }))
}

pub fn convert_py_cues_to_cues(
    py_translated_cues: Py<PyAny>,
    subtitle_document: &SubtitleDocument,
) -> Result<Vec<SubtitleCue>, TranslationError> {
    Python::attach(|py| -> PyResult<Vec<SubtitleCue>> {
        let translated_cues = py_translated_cues.bind(py).cast::<PyList>()?;
        translated_cues
            .iter()
            .enumerate()
            .map(|(i, cue)| {
                let start = timedelta_to_duration(&cue.getattr("start")?)?;
                let end = timedelta_to_duration(&cue.getattr("end")?)?;
                let content = cue.getattr("content")?.extract()?;

                // Need to check that length of subtitle_document.cues
                // is the same as the length of aligned_cues
                Ok(SubtitleCue {
                    id: subtitle_document.cues[i].id,
                    start,
                    end,
                    content: SubtitleContent::Text(content),
                })
            })
            .collect()
    })
    .map_err(|e| TranslationError::PythonError { error: e })
}

pub fn generate_translated_metadata(
    subtitle_document: SubtitleDocument,
    language: Language,
) -> SubtitleMetadata {
    SubtitleMetadata {
        album: subtitle_document.metadata.album,
        title: subtitle_document.metadata.title,
        artists: subtitle_document.metadata.artists,
        languages: subtitle_document.metadata.languages,
        file_path: match subtitle_document.metadata.file_path {
            Some(path) => {
                let stem = path.file_stem().unwrap().to_string_lossy();
                let language_code_2 = language.as_code_2();
                let new_path = PathBuf::from(format!("{stem}.{language_code_2}.lrc"));
                Some(new_path)
            }
            None => None,
        },
    }
}
