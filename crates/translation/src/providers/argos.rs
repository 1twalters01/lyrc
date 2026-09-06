use std::path::PathBuf;

use futures::future::BoxFuture;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use pyo3_async_runtimes::tokio::into_future;
use subtitles::{
    language::Language,
    subtitles::{SubtitleContent, SubtitleCue, SubtitleDocument, SubtitleMetadata},
};

use crate::{error::TranslationError, helpers::timedelta_to_duration, provider::LyricsTranslator};

pub struct ArgosTranslator;

impl LyricsTranslator for ArgosTranslator {
    fn translate(
        &self,
        language: Language,
        subtitle_document: SubtitleDocument,
    ) -> BoxFuture<'_, Result<Option<SubtitleDocument>, TranslationError>> {
        Box::pin(async move {
            let original_language = subtitle_document
                .metadata
                .languages
                .first()
                .ok_or(TranslationError::NoLanguageCode)?;

            let py_future = Python::attach(|py| -> PyResult<_> {
                let datetime = py.import("datetime")?;
                let timedelta = datetime.getattr("timedelta")?;

                let service_module = PyModule::import(py, "translator.service")?;
                let provider_module = PyModule::import(py, "translator.argos.provider")?;
                let options_module = PyModule::import(py, "translator.argos.options")?;
                let cue_module = PyModule::import(py, "translator.models.cue")?;
                let language_module = PyModule::import(py, "translator.models.language")?;

                let original_language_py = language_module.getattr("Language")?.call1((
                    original_language.as_name(),
                    original_language.as_native_name(),
                    original_language.as_code_2(),
                    original_language.as_code_3(),
                ))?;
                let to_language_py = language_module.getattr("Language")?.call1((
                    language.as_name(),
                    language.as_native_name(),
                    language.as_code_2(),
                    language.as_code_3(),
                ))?;

                let options = options_module
                    .getattr("ArgosOptions")?
                    .call1((original_language_py.clone(),))?;

                let argos_translator = provider_module.getattr("ArgosTranslator")?.call0()?;

                argos_translator
                    .getattr("install_packages")?
                    .call1((to_language_py.clone(), original_language_py))?;

                let providers = PyDict::new(py);
                providers.set_item("argos", argos_translator)?;

                let translation_service = service_module
                    .getattr("TranslationService")?
                    .call1((providers,))?;

                let lrc_contents = PyList::empty(py);
                for cue in &subtitle_document.cues {
                    let start =
                        timedelta.call1((0, 0, cue.start.num_microseconds().unwrap_or(0)))?;
                    let end =
                        timedelta.call1((0, 0, cue.end.num_microseconds().unwrap_or(0)))?;

                    let content = match &cue.content {
                        SubtitleContent::Text(text) => text.clone(),
                        SubtitleContent::Words(words) => {
                            let mut content = String::new();
                            for word in words {
                                content.push_str(&word.content);
                            }
                            
                            content
                        }
                    };

                    let py_cue = cue_module
                        .getattr("Cue")?
                        .call1((start, end, content))?;

                    lrc_contents.append(py_cue)?;
                }

                let coroutine = translation_service.call_method1(
                    "translate",
                    ("argos", lrc_contents, to_language_py, options),
                )?;

                into_future(coroutine)
            })?;

            let translated_cues = py_future
                .await?;

            let translated_cues = Python::attach(|py| -> PyResult<Vec<SubtitleCue>> {
                let translated_cues = translated_cues.bind(py).cast::<PyList>()?;
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
            .map_err(|e| TranslationError::PythonError { error: e })?;

            let translated_metadata = SubtitleMetadata {
                album: subtitle_document.metadata.album,
                title: subtitle_document.metadata.title,
                artists: subtitle_document.metadata.artists,
                languages: subtitle_document.metadata.languages,
                file_path: match subtitle_document.metadata.file_path {
                    Some(mut path) => {
                        let stem = path.file_stem().unwrap().to_string_lossy();
                        let language_code_2 = language.as_code_2();
                        let new_path = PathBuf::from(format!("{stem}.{language_code_2}.lrc"));
                        Some(new_path)
                    }
                    None => None,
                },
            };

            let translated_subtitle_document = SubtitleDocument {
                metadata: translated_metadata,
                cues: translated_cues,
            };

            Ok(Some(translated_subtitle_document))
        })
    }
}
