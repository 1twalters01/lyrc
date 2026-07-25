use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

pub trait Synchronizer {
    type Event;

    fn update(
        &mut self,
        subtitle_document: &SubtitleDocument,
        position: &Duration,
    ) -> Option<Self::Event>;
}
