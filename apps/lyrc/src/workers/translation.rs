use std::thread;

use translation::{
    messages::{TranslationRequest, TranslationResult, TranslationTask},
    provider::LyricsTranslator,
    providers::argos::ArgosTranslator,
};

pub async fn start_translation_worker(
    mut request_rx: tokio::sync::mpsc::Receiver<TranslationRequest>,
    result_tx: tokio::sync::mpsc::Sender<TranslationResult>,
) {
    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async move {
            while let Some(request) = request_rx.recv().await {
                match request {
                    TranslationRequest::Translate(TranslationTask {
                        language,
                        subtitle_document,
                    }) => {
                        let argos_result = ArgosTranslator.translate(language, subtitle_document);

                        let result = match argos_result.await {
                            Ok(subtitle_document) => TranslationResult::Complete(subtitle_document),
                            Err(error) => TranslationResult::Failed(error),
                        };

                        // decide to use sync or async send
                        // if result_tx.blocking_send(result).is_err() {
                        if result_tx.send(result).await.is_err() {
                            break;
                        }
                    } /* TranslationRequest::Cancel => {
                       * }, */
                }
            }
        })
    });
}
