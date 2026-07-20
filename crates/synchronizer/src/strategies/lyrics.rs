use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;

use crate::traits::Synchronizer;

pub enum LyricsSyncEvent {
    Changed {
        old_cues: Vec<usize>,
        new_cues: Vec<usize>,
    },
}

pub struct LyricsSynchronizer {
    active_cues: Vec<usize>,
}

impl Synchronizer for LyricsSynchronizer {
    type Event = LyricsSyncEvent;

    fn update(&mut self, subtitles: SubtitleDocument, position: Duration) -> Option<Self::Event> {
        let new_cues = Self::get_cues_at(&subtitles, position);

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
}

impl LyricsSynchronizer {
    fn get_cues_at(subtitle_document: &SubtitleDocument, position: Duration) -> Vec<usize> {
        let start = subtitle_document
            .cues
            .partition_point(|cue| cue.start <= position);

        subtitle_document.cues[..start]
            .iter()
            .enumerate()
            .filter_map(|(index, cue)| {
                if cue.end.map_or(true, |end| position < end) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }
}
