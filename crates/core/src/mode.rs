use std::fmt::Display;

use subtitles::subtitles::SubtitleContent;

#[derive(Clone, PartialEq)]
pub struct EditCue {
    pub index: usize,
    pub original_content: SubtitleContent,
}

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Select {
        cue_index: usize,
        selected_cues: Vec<usize>,
    },
    Edit {
        cue_index: usize,
        selected_cues: Vec<EditCue>,
    },
}

impl Display for AppMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Select {
                cue_index,
                selected_cues: _,
            } => write!(f, "select cue: {:?}", cue_index),
            Self::Edit {
                cue_index,
                selected_cues: _,
            } => write!(f, "edit cue: {:?}", cue_index),
        }
    }
}
