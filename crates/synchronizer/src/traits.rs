use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

pub trait Synchronizer {
    type Event;

    fn update(&mut self, subtitles: SubtitleDocument, position: Duration) -> Option<Self::Event>;
}
