use std::path::PathBuf;

use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use subtitles::subtitles::{
    AlignedWord, SubtitleContent, SubtitleCue, SubtitleDocument, SubtitleMetadata,
};

use crate::{
    helpers::timedelta_to_duration,
    provider::LyricsAligner,
    error::AlignmentError,
};

pub struct WhisperXAligner;

impl LyricsAligner for WhisperXAligner {
    fn align_cues(
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
            .as_code_2();
        let device = "cuda"; // Store in Config crate

        let aligned_cues = Python::attach(|py| -> Result<Py<PyAny>, AlignmentError> {
            let datetime = py.import("datetime")?;
            let timedelta = datetime.getattr("timedelta")?;

            let service_module = PyModule::import(py, "aligner.service")?;
            let provider_module = PyModule::import(py, "aligner.whisperx.provider")?;
            let options_module = PyModule::import(py, "aligner.whisperx.options")?;
            let models_module = PyModule::import(py, "aligner.models.cue")?;

            let whisperx_aligner = provider_module
                .getattr("WhisperXAligner")?
                .call1((device,))?;
            let providers = PyDict::new(py);
            providers.set_item("whisperx", whisperx_aligner)?;

            let alignment_service = service_module
                .getattr("AlignmentService")?
                .call1((providers,))?;

            let options = options_module
                .getattr("WhisperXOptions")?
                .call1((language_code,))?;

            let lrc_contents = subtitle_document
                .cues
                .iter()
                .map(|cue| {
                    let start =
                        timedelta.call1((0, 0, cue.start.num_microseconds().unwrap_or(0)))?;
                    let end = timedelta.call1((0, 0, cue.end.num_microseconds().unwrap_or(0)))?;

                    let content = match &cue.content {
                        SubtitleContent::Text(text) => text,
                        SubtitleContent::Words(_words) => {
                            return Err(AlignmentError::AlreadyAligned);
                        }
                    };

                    Ok(models_module.getattr("Cue")?.call1((start, end, content))?)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let result = alignment_service.call_method1(
                "align_cues",
                ("whisperx", lrc_contents, audio_path, options),
            )?;

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
        .map_err(|e| AlignmentError::PythonError { error: e })?;

        let aligned_metadata = SubtitleMetadata {
            album: subtitle_document.metadata.album,
            title: subtitle_document.metadata.title,
            artists: subtitle_document.metadata.artists,
            languages: subtitle_document.metadata.languages,
            file_path: match subtitle_document.metadata.file_path {
                Some(mut path) => {
                    path.set_extension(".elrc");
                    Some(path)
                }
                None => None,
            },
        };

        let aligned_subtitle_document = SubtitleDocument {
            metadata: aligned_metadata,
            cues: aligned_cues,
        };

        Ok(Some(aligned_subtitle_document))
    }
}
