use chrono::Duration;
use subtitles::subtitles::{SubtitleContent, SubtitleCue};

#[derive(Clone)]
pub struct EditHistory {
    undo: Vec<Edit>,
    redo: Vec<Edit>,
}

impl EditHistory {
    pub fn new() -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    pub fn push(&mut self, edit: Edit) {
        self.undo.push(edit);
        self.redo.clear();
    }

    pub fn push_undo(&mut self, edit: Edit) {
        self.undo.push(edit);
    }

    pub fn pop_undo(&mut self) -> Option<Edit> {
        self.undo.pop()
    }

    pub fn push_redo(&mut self, edit: Edit) {
        self.redo.push(edit);
    }

    pub fn pop_redo(&mut self) -> Option<Edit> {
        self.redo.pop()
    }
}

#[derive(Clone)]
pub enum Edit {
    EditCueContent { changes: Vec<CueContentChange> },
    EditCueTimes { changes: Vec<CueTimeChange> },
    DeleteCue { cues: Vec<IndexedSubtitleCue> },
    InsertCue { cues: Vec<IndexedSubtitleCue> },
}

#[derive(Clone)]
pub struct CueContentChange {
    pub index: usize,
    pub old_content: SubtitleContent,
    pub new_content: SubtitleContent,
}

#[derive(Clone)]
pub struct CueTimeChange {
    new_index: usize,
    old_index: usize,
    old_start: Duration,
    old_end: Duration,
    new_start: Duration,
    new_end: Duration,
}

#[derive(Clone)]
pub struct IndexedSubtitleCue {
    index: usize,
    subtitle_cue: SubtitleCue,
}
