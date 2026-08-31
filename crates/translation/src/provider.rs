use subtitles::{language::Language, subtitles::SubtitleDocument};

use crate::error::TranslationError;

pub trait LyricsAligner: Send + Sync {
    fn translate(
        language: Language,
        subtitle_document: SubtitleDocument,
    ) -> Result<Option<SubtitleDocument>, TranslationError>;
}
