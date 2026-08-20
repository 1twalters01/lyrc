use std::path::PathBuf;

use pyo3::{prelude::*, types::PyList};
use subtitles::subtitles::{AlignedWord, SubtitleContent, SubtitleCue, SubtitleDocument};

use crate::{
    helpers::timedelta_to_duration,
    provider::{AlignmentError, LyricsAligner},
};

pub struct WhisperXAligner;

impl LyricsAligner for WhisperXAligner {
    fn align_cues(
        // &self,
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
        





        Ok(Some(subtitle_document))
    }
}
