use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

use crate::strategies::{cues::CueIndex, words::WordIndex};

pub trait CueIndexed {
    fn cue_index(&self) -> CueIndex;
}

pub trait ActiveIndexed: CueIndexed {
    fn word_index(&self) -> Option<WordIndex> {
        None
    }
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
