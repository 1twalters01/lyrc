use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

use crate::traits::{ActiveIndexed, CueIndexed, Synchronizer};

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

impl ActiveIndexed for CueIndex {}

pub enum CueSyncEvent {
    Changed {
        old_cues: Vec<CueIndex>,
        new_cues: Vec<CueIndex>,
    },
}

pub struct CueSynchronizer {
    active_cues: Vec<CueIndex>,
}

impl Synchronizer for CueSynchronizer {
    type Event = CueSyncEvent;
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

            let event = CueSyncEvent::Changed {
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

impl CueSynchronizer {
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
