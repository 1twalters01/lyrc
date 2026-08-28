use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

use crate::traits::{CueIndexed, Synchronizer};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CueIndex {
    pub cue: usize,
}

impl Default for CueIndex {
    fn default() -> Self {
        Self { cue: 0 }
    }
}

impl CueIndexed for CueIndex {
    fn cue_index(&self) -> Self {
        *self
    }
}

pub enum LyricsSyncEvent {
    Changed {
        old_cues: Vec<CueIndex>,
        new_cues: Vec<CueIndex>,
    },
}

pub struct LyricsSynchronizer {
    active_cues: Vec<CueIndex>,
}

impl Synchronizer for LyricsSynchronizer {
    type Event = LyricsSyncEvent;
    type Active = CueIndex;

    fn update(
        &mut self,
        subtitle_document: &Option<SubtitleDocument>,
        position: &Option<Duration>,
    ) -> Option<Self::Event> {
        let (subtitle_document, position) = match (subtitle_document, position) {
            (Some(subtitle_document), Some(position)) => (subtitle_document, position),
            (_, _) => return None,
        };

        let new_cues = Self::get_cues_at(&subtitle_document, Some(position));

        if new_cues != self.active_cues {
            let old_cues = std::mem::replace(&mut self.active_cues, new_cues);

            let event = LyricsSyncEvent::Changed {
                old_cues,
                new_cues: self.active_cues.clone(),
            };

            return Some(event);
        }

        None
    }

    fn get_active_indices(&self) -> &[CueIndex] {
        &self.active_cues
    }
}

impl LyricsSynchronizer {
    pub fn new() -> Self {
        Self {
            active_cues: Vec::new(),
        }
    }

    pub fn get_cues_at(
        subtitle_document: &SubtitleDocument,
        position: Option<&Duration>,
    ) -> Vec<CueIndex> {
        let position = match position {
            Some(position) => position,
            None => return Vec::new(),
        };

        let start = subtitle_document
            .cues
            .partition_point(|cue| &cue.start <= position);

        subtitle_document.cues[..start]
            .iter()
            .enumerate()
            .filter_map(|(index, cue)| {
                if position < &cue.end {
                    Some(CueIndex { cue: index })
                } else {
                    None
                }
            })
            .collect()
    }
}
