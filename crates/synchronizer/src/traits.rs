use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

pub trait Synchronizer {
    type Event;

    fn update(
        &mut self,
        subtitle_document: &Option<SubtitleDocument>,
        position: &Option<Duration>,
    ) -> Option<Self::Event>;

    fn get_active_cues(&self) -> &[usize];
}
