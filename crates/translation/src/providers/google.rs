use subtitles::{language::Language, subtitles::SubtitleDocument};

use crate::{error::TranslationError, provider::LyricsTranslator};

pub struct GoogleTranslator;

impl LyricsTranslator for GoogleTranslator {
    fn translate(
        &self,
        language: Language,
        subtitle_document: SubtitleDocument,
    ) -> futures::future::BoxFuture<'_, Result<Option<SubtitleDocument>, TranslationError>> {
        todo!()
    }
}
