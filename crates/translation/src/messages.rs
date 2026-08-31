use subtitles::{language::Language, subtitles::SubtitleDocument};

pub enum TranslationRequest {
    Align(TranslationTask),
    // Cancel,
}

pub struct TranslationTask {
    pub language: Language,
    pub subtitle_document: SubtitleDocument,
}

// Change error to not be string
#[derive(Debug)]
pub enum TranslationResult {
    Complete(Option<SubtitleDocument>),
    Cancelled,
    Failed(String),
}
