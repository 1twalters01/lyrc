use chrono::Duration;
use synchronizer::{
    strategies::{
        cues::{CueIndex, CueSyncEvent, CueSynchronizer},
        words::{WordIndex, WordSyncEvent, WordSynchronizer},
    },
    traits::{CueIndexed, Synchronizer},
};

#[derive(Clone, Debug)]
pub enum SyncEvent {
    Word(WordSyncEvent),
    Cue(CueSyncEvent),
}

#[derive(Clone, Debug)]
pub enum ActiveIndex {
    Cue(CueIndex),
    Word(WordIndex),
}

impl ActiveIndex {
    pub fn cue_index(&self) -> CueIndex {
        match self {
            Self::Word(w) => w.cue_index(),
            Self::Cue(c) => c.cue_index(),
        }
    }
}

impl Default for ActiveIndex {
    fn default() -> Self {
        Self::Cue(CueIndex::default())
    }
}

pub enum SynchronizerMode {
    Cue,
    Word,
}

pub struct AppSynchronizer {
    pub cue_synchronizer: CueSynchronizer,
    pub word_synchronizer: WordSynchronizer,
    pub mode: SynchronizerMode,
}

impl AppSynchronizer {
    pub fn new() -> Self {
        let cue_synchronizer = CueSynchronizer::new();
        let word_synchronizer = WordSynchronizer::new();

        Self {
            cue_synchronizer,
            word_synchronizer,
            mode: SynchronizerMode::Cue,
        }
    }

    pub fn get_active_indices(&self) -> Vec<ActiveIndex> {
        match self.mode {
            SynchronizerMode::Word => self
                .word_synchronizer
                .get_active_indices()
                .iter()
                .map(|index| ActiveIndex::Word(*index))
                .collect(),
            SynchronizerMode::Cue => self
                .cue_synchronizer
                .get_active_indices()
                .iter()
                .map(|index| ActiveIndex::Cue(*index))
                .collect(),
        }
    }

    pub fn update(
        &mut self,
        subtitle_document: &Option<subtitles::subtitles::SubtitleDocument>,
        position: &Option<Duration>,
    ) -> Option<SyncEvent> {
        let event = match self.mode {
            SynchronizerMode::Word => self
                .word_synchronizer
                .update(subtitle_document, position)
                .map(|e| SyncEvent::Word(e)),
            SynchronizerMode::Cue => self
                .cue_synchronizer
                .update(subtitle_document, position)
                .map(|e| SyncEvent::Cue(e)),
        };
        return event;
    }
}
