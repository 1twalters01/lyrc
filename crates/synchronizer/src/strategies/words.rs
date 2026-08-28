use std::usize;

use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

use crate::{
    strategies::lyrics::CueIndex,
    traits::{CueIndexed, Synchronizer},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WordIndex {
    pub cue: usize,
    pub word: usize,
}

impl Default for WordIndex {
    fn default() -> Self {
        Self { cue: 0, word: 0 }
    }
}

impl CueIndexed for WordIndex {
    fn cue_index(&self) -> super::lyrics::CueIndex {
        CueIndex { cue: self.cue }
    }
}

pub enum WordSyncEvent {
    Changed {
        old_words: Vec<WordIndex>,
        new_words: Vec<WordIndex>,
    },
}

pub struct WordSynchronizer {
    active_words: Vec<WordIndex>,
}

impl Synchronizer for WordSynchronizer {
    type Event = WordSyncEvent;
    type Active = WordIndex;

    fn update(
        &mut self,
        subtitle_document: &Option<SubtitleDocument>,
        position: &Option<Duration>,
    ) -> Option<Self::Event> {
        None
    }

    fn get_active_indices(&self) -> &[WordIndex] {
        &self.active_words
    }
}
