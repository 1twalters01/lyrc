use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

use crate::strategies::lyrics::CueIndex;

pub trait CueIndexed {
    fn cue_index(&self) -> CueIndex;
}

pub trait Synchronizer {
    type Event;
    type Active: Copy + Default + CueIndexed;

    fn update(
        &mut self,
        subtitle_document: &Option<SubtitleDocument>,
        position: &Option<Duration>,
    ) -> Option<Self::Event>;

    fn get_active_indices(&self) -> &[Self::Active];
}
