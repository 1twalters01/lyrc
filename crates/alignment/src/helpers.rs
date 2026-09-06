use chrono::Duration;
use pyo3::{prelude::*, types::PyList};
use subtitles::subtitles::{
    AlignedWord, SubtitleContent, SubtitleCue, SubtitleDocument, SubtitleMetadata,
};

use crate::error::AlignmentError;

pub fn timedelta_to_duration(timedelta: &Bound<'_, PyAny>) -> PyResult<Duration> {
    let days: i64 = timedelta.getattr("days")?.extract()?;
    let seconds: i64 = timedelta.getattr("seconds")?.extract()?;
    let microseconds: i64 = timedelta.getattr("microseconds")?.extract()?;

    Ok(chrono::Duration::days(days)
        + chrono::Duration::seconds(seconds)
        + chrono::Duration::microseconds(microseconds))
}

pub fn convert_py_cues_to_line_aligned_subtitle_document(
    py_aligned_cues: Py<PyAny>,
    subtitle_document: SubtitleDocument,
) -> Result<Option<SubtitleDocument>, AlignmentError> {
    let aligned_cues = convert_py_cues_to_line_level_cues(py_aligned_cues, &subtitle_document)?;

    let aligned_metadata = generate_aligned_metadata(subtitle_document);

    Ok(Some(SubtitleDocument {
        metadata: aligned_metadata,
        cues: aligned_cues,
    }))
}

pub fn convert_py_cues_to_word_aligned_subtitle_document(
    py_aligned_cues: Py<PyAny>,
    subtitle_document: SubtitleDocument,
) -> Result<Option<SubtitleDocument>, AlignmentError> {
    let aligned_cues = convert_py_cues_to_word_level_cues(py_aligned_cues, &subtitle_document)?;

    let aligned_metadata = generate_aligned_metadata(subtitle_document);

    Ok(Some(SubtitleDocument {
        metadata: aligned_metadata,
        cues: aligned_cues,
    }))
}

pub fn convert_py_cues_to_word_level_cues(
    py_aligned_cues: Py<PyAny>,
    subtitle_document: &SubtitleDocument,
) -> Result<Vec<SubtitleCue>, AlignmentError> {
    Python::attach(|py| -> PyResult<Vec<SubtitleCue>> {
        let aligned_cues = py_aligned_cues.bind(py).cast::<PyList>()?;
        aligned_cues
            .iter()
            .enumerate()
            .map(|(i, cue)| {
                let start = timedelta_to_duration(&cue.getattr("start")?)?;
                let end = timedelta_to_duration(&cue.getattr("end")?)?;
                let words = cue
                    .getattr("words")?
                    .cast::<PyList>()?
                    .iter()
                    .map(|word| {
                        Ok(AlignedWord {
                            start: timedelta_to_duration(&word.getattr("start")?)?,
                            end: timedelta_to_duration(&word.getattr("end")?)?,
                            content: word.getattr("text")?.extract()?,
                        })
                    })
                    .collect::<PyResult<Vec<_>>>()?;

                // Need to check that length of subtitle_document.cues
                // is the same as the length of aligned_cues
                Ok(SubtitleCue {
                    id: subtitle_document.cues[i].id,
                    start,
                    end,
                    content: SubtitleContent::Words(words),
                })
            })
            .collect()
    })
    .map_err(|e| AlignmentError::PythonError { error: e })
}

pub fn convert_py_cues_to_line_level_cues(
    py_translated_cues: Py<PyAny>,
    subtitle_document: &SubtitleDocument,
) -> Result<Vec<SubtitleCue>, AlignmentError> {
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
    .map_err(|e| AlignmentError::PythonError { error: e })
}

pub fn generate_aligned_metadata(subtitle_document: SubtitleDocument) -> SubtitleMetadata {
    SubtitleMetadata {
        album: subtitle_document.metadata.album,
        title: subtitle_document.metadata.title,
        artists: subtitle_document.metadata.artists,
        languages: subtitle_document.metadata.languages,
        file_path: match subtitle_document.metadata.file_path {
            Some(mut path) => {
                path.set_extension("elrc");
                Some(path)
            }
            None => None,
        },
    }
}
