use futures::future::BoxFuture;
use subtitles::{language::Language, subtitles::SubtitleDocument};

use crate::error::TranslationError;

pub trait LyricsTranslator: Send + Sync {
    fn translate(
        &self,
        language: Language,
        subtitle_document: SubtitleDocument,
    ) -> BoxFuture<'_, Result<Option<SubtitleDocument>, TranslationError>>;
}
