use crate::workers::{alignment::AlignmentWorker, translation::TranslationWorker};

pub struct Workers {
    pub alignment: AlignmentWorker,
    pub translation: TranslationWorker,
}

impl Workers {
    pub async fn start() -> Self {
        let alignment = AlignmentWorker::new();
        let translation = TranslationWorker::new().await;

        Self {
            alignment,
            translation,
        }
    }
}
