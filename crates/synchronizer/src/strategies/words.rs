use std::usize;

use chrono::Duration;
use subtitles::subtitles::{SubtitleContent, SubtitleDocument};

use crate::{
    strategies::cues::{CueIndex, CueSynchronizer},
    traits::{ActiveIndexed, CueIndexed, Synchronizer},
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
    fn cue_index(&self) -> CueIndex {
        CueIndex { cue: self.cue }
    }
}

impl ActiveIndexed for WordIndex {
    fn word_index(&self) -> Option<WordIndex> {
        Some(*self)
    }
}

#[derive(Clone, Debug)]
pub enum WordSyncEvent {
    Changed {
        old_words: Vec<WordIndex>,
        new_words: Vec<WordIndex>,
    },
}

pub struct WordSynchronizer {
    active_words: Vec<WordIndex>,
    last_position: Option<Duration>,
}

impl Synchronizer for WordSynchronizer {
    type Event = WordSyncEvent;
    type Active = WordIndex;

    fn update(
        &mut self,
        subtitle_document: &Option<SubtitleDocument>,
        position: &Option<Duration>,
    ) -> Option<Self::Event> {
        let (subtitle_document, position) = match (subtitle_document, position) {
            (Some(subtitle_document), Some(position)) => (subtitle_document, position),
            (_, _) => return None,
        };

        let is_seeking_backwards = self.last_position.is_some_and(|last| position < &last);

        let mut new_words = Self::get_words_at(&subtitle_document, Some(position));
        new_words = if new_words.is_empty() && !is_seeking_backwards {
            self.active_words.clone()
        } else {
            new_words
        };

        if new_words != self.active_words {
            let old_words = std::mem::replace(&mut self.active_words, new_words);

            let event = WordSyncEvent::Changed {
                old_words,
                new_words: self.active_words.clone(),
            };

            return Some(event);
        }

        None
    }

    fn get_active_indices(&self) -> &[WordIndex] {
        &self.active_words
    }
}

impl WordSynchronizer {
    pub fn new() -> Self {
        Self {
            active_words: Vec::new(),
            last_position: None,
        }
    }

    pub fn get_words_at(
        subtitle_document: &SubtitleDocument,
        position: Option<&Duration>,
    ) -> Vec<WordIndex> {
        let position = match position {
            Some(position) => position,
            None => return Vec::new(),
        };

        let current_cue_indices = CueSynchronizer::get_cues_at(subtitle_document, Some(position));

        current_cue_indices
            .iter()
            .flat_map(|cue_index| {
                let cue = &subtitle_document.cues[cue_index.cue];

                match &cue.content {
                    SubtitleContent::Text(_) => Vec::new().into_iter(),

                    SubtitleContent::Words(words) => words
                        .iter()
                        .enumerate()
                        .filter_map(|(word_index, word)| {
                            if word.start <= *position && *position < word.end {
                                Some(WordIndex {
                                    cue: cue_index.cue,
                                    word: word_index,
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .into_iter(),
                }
            })
            .collect()
    }
}
