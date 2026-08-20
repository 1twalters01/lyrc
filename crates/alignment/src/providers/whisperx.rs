use futures::future::BoxFuture;
use mpris::track::Track;
use subtitles::subtitles::SubtitleDocument;

use crate::provider::{AlignmentError, LyricsAligner};

pub struct WhisperXAligner;

impl LyricsAligner for WhisperXAligner {
    fn align_cues(&self, track: Track, subtitle_document: SubtitleDocument) -> BoxFuture<'static, Result<Option<SubtitleDocument>, AlignmentError>> {
        Box::pin(async move {
        
            Ok(Some(subtitle_document))
        })
    }
}
