use std::path::PathBuf;

use pyo3::{prelude::*, types::PyDict};
use subtitles::{
    language::Language,
    subtitles::{SubtitleContent, SubtitleDocument},
};

use crate::{
    error::AlignmentError, helpers::convert_py_cues_to_line_aligned_subtitle_document,
    provider::LyricsAligner,
};

pub struct AeneasAligner;

impl LyricsAligner for AeneasAligner {
    fn align_cues(
        // &self,
        audio_file_path: PathBuf,
        subtitle_document: SubtitleDocument,
    ) -> Result<Option<SubtitleDocument>, AlignmentError> {
        let audio_path = audio_file_path
            .to_str()
            .ok_or(AlignmentError::InvalidAudioPath)?
            .to_owned();

        let language = subtitle_document
            .metadata
            .languages
            .first()
            .ok_or(AlignmentError::NoLanguageCode)?;

        let py_aligned_cues = Self::align_cues(&subtitle_document, audio_path, language)?;

        convert_py_cues_to_line_aligned_subtitle_document(py_aligned_cues, subtitle_document)
    }
}

impl AeneasAligner {
    fn align_cues(
        subtitle_document: &SubtitleDocument,
        audio_path: String,
        language: &Language,
        // device: &str,
    ) -> Result<Py<PyAny>, AlignmentError> {
        Python::attach(|py| -> Result<Py<PyAny>, AlignmentError> {
            let datetime = py.import("datetime")?;
            let timedelta = datetime.getattr("timedelta")?;

            let provider_module = PyModule::import(py, "aligner.aeneas.provider")?;
            let options_module = PyModule::import(py, "aligner.aeneas.options")?;
            let service_module = PyModule::import(py, "aligner.service")?;
            let language_module = PyModule::import(py, "aligner.models.language")?;
            let cue_module = PyModule::import(py, "aligner.models.cue")?;

            let aeneas_aligner = provider_module.getattr("AeneasAligner")?.call1(())?;
            let providers = PyDict::new(py);
            providers.set_item("aeneas", aeneas_aligner)?;

            let alignment_service = service_module
                .getattr("AlignmentService")?
                .call1((providers,))?;

            let language_py = language_module.getattr("Language")?.call1((
                language.as_name(),
                language.as_native_name(),
                language.as_code_2(),
                language.as_code_3(),
                language.as_flores_200(),
            ))?;
            let options = options_module
                .getattr("AeneasOptions")?
                .call1((language_py,))?;

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

                    Ok(cue_module.getattr("Cue")?.call1((start, end, content))?)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let result = alignment_service
                .call_method1("align_cues", ("aeneas", lrc_contents, audio_path, options))?;

            Ok(result.unbind())
        })
    }
}
