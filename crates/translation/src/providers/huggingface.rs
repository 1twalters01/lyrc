use subtitles::{language::Language, subtitles::SubtitleDocument};

use crate::{error::TranslationError, provider::LyricsTranslator};

pub struct HuggingfaceTranslator;

impl LyricsTranslator for HuggingfaceTranslator {
    fn translate(
        &self,
        language: Language,
        subtitle_document: SubtitleDocument,
    ) -> futures::future::BoxFuture<'_, Result<Option<SubtitleDocument>, TranslationError>> {
        todo!()
    }
}
