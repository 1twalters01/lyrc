use std::path::PathBuf;

use pyo3::{prelude::*, types::PyList};
use subtitles::subtitles::{AlignedWord, SubtitleContent, SubtitleCue, SubtitleDocument};

use crate::provider::{AlignmentError, LyricsAligner};

pub struct WhisperXAligner;

impl LyricsAligner for WhisperXAligner {
    fn align_cues(
        &self,
        audio_file_path: PathBuf,
        subtitle_document: SubtitleDocument,
    ) -> Result<Option<SubtitleDocument>, AlignmentError> {
        let audio_path = audio_file_path
            .to_str()
            .ok_or(AlignmentError::InvalidAudioPath)?
            .to_owned();

        // Get the language code from the track instead?
        // Would need to use mpris::track::Track in that case
        let language_code = subtitle_document
            .metadata
            .languages
            .first()
            .ok_or(AlignmentError::NoLanguageCode)?
            .clone();
        let device = "cuda"; // Store in Config crate

        let aligned_cues = Python::attach(|py| -> Result<Py<PyAny>, AlignmentError> {
            let provider_module = PyModule::import(py, "alignment.providers.whisperx")
                .map_err(|e| AlignmentError::PythonError { error: e })?;
            let models_module = PyModule::import(py, "alignment.models.cue")
                .map_err(|e| AlignmentError::PythonError { error: e })?;
            let datetime = py
                .import("datetime")
                .map_err(|e| AlignmentError::PythonError { error: e })?;
            let timedelta = datetime
                .getattr("timedelta")
                .map_err(|e| AlignmentError::PythonError { error: e })?;

            let lrc_contents = subtitle_document
                .cues
                .iter()
                .map(|cue| {
                    let start = timedelta
                        .call1((0, 0, cue.start.num_microseconds().unwrap_or(0)))
                        .map_err(|e| AlignmentError::PythonError { error: e })?;
                    let end = timedelta
                        .call1((0, 0, cue.end.num_microseconds().unwrap_or(0)))
                        .map_err(|e| AlignmentError::PythonError { error: e })?;

                    let content = match &cue.content {
                        SubtitleContent::Text(text) => text,
                        SubtitleContent::Words(_words) => {
                            return Err(AlignmentError::AlreadyAligned);
                        }
                    };

                    Ok(models_module
                        .getattr("Cue")
                        .map_err(|e| AlignmentError::PythonError { error: e })?
                        .call1((start, end, content))
                        .map_err(|e| AlignmentError::PythonError { error: e })?)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let whisperx_aligner = provider_module
                .getattr("WhisperXAligner")
                .map_err(|e| AlignmentError::PythonError { error: e })?
                .call0()
                .map_err(|e| AlignmentError::PythonError { error: e })?;

            let result = whisperx_aligner
                .call_method1(
                    "align_cues",
                    (lrc_contents, audio_path, language_code, device),
                )
                .map_err(|e| AlignmentError::PythonError { error: e })?;

            Ok(result.unbind())
        })?;

        let aligned_cues = Python::attach(|py| -> PyResult<Vec<SubtitleCue>> {
            let aligned_cues = aligned_cues.bind(py).cast::<PyList>()?;
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

                    Ok(SubtitleCue {
                        id: subtitle_document.cues[i].id,
                        start,
                        end,
                        content: SubtitleContent::Words(words),
                    })
                })
                .collect()
        })
        .map_err(|e| AlignmentError::PythonError { error: e })?;

        let aligned_subtitle_document = SubtitleDocument {
            metadata: subtitle_document.metadata.clone(),
            cues: aligned_cues,
        };

        Ok(Some(aligned_subtitle_document))
    }
}

fn timedelta_to_duration(timedelta: &Bound<'_, PyAny>) -> PyResult<chrono::Duration> {
    let days: i64 = timedelta.getattr("days")?.extract()?;
    let seconds: i64 = timedelta.getattr("seconds")?.extract()?;
    let microseconds: i64 = timedelta.getattr("microseconds")?.extract()?;

    Ok(chrono::Duration::days(days)
        + chrono::Duration::seconds(seconds)
        + chrono::Duration::microseconds(microseconds))
}
