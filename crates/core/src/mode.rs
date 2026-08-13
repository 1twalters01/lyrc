use std::fmt::Display;

use subtitles::subtitles::SubtitleContent;

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Select {
        cue_index: usize,
    },
    Edit {
        cue_index: usize,
        original_content: SubtitleContent,
    },
}

impl Display for AppMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Select { cue_index } => write!(f, "select cue: {:?}", cue_index),
            Self::Edit {
                cue_index,
                original_content: _,
            } => write!(f, "edit cue: {:?}", cue_index),
        }
    }
}

