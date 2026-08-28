use std::thread;

use alignment::{
    messages::{AlignmentRequest, AlignmentResult, AlignmentTask},
    provider::LyricsAligner,
    providers::whisperx::WhisperXAligner,
};

pub fn start_alignment_worker(
    mut request_rx: tokio::sync::mpsc::Receiver<AlignmentRequest>,
    result_tx: tokio::sync::mpsc::Sender<AlignmentResult>,
) {
    thread::spawn(move || {
        while let Some(request) = request_rx.blocking_recv() {
            match request {
                AlignmentRequest::Align(AlignmentTask {
                    audio_file_path,
                    subtitle_document,
                }) => {
                    let whisperx_result =
                        WhisperXAligner::align_cues(audio_file_path, subtitle_document);

                    let result = match whisperx_result {
                        Ok(subtitle_document) => AlignmentResult::Complete(subtitle_document),
                        Err(error) => AlignmentResult::Failed(error.to_string()),
                    };

                    if result_tx.blocking_send(result).is_err() {
                        break;
                    }
                }
                // AlignmentRequest::Cancel => {}
            }
        }
    });
}
