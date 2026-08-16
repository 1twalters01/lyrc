use chrono::Duration;
use subtitles::subtitles::{SubtitleContent, SubtitleCue};
use uuid::Uuid;

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

    pub fn is_empty(&self) -> bool {
        self.undo.is_empty()
    }

    pub fn empty(&mut self) {
        self.undo.clear();
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
    pub id: Uuid,
    pub new_index: usize,
    pub old_index: usize,
    pub old_start: Duration,
    pub old_end: Duration,
    pub new_start: Duration,
    pub new_end: Duration,
}

#[derive(Clone)]
pub struct IndexedSubtitleCue {
    pub index: usize,
    pub subtitle_cue: SubtitleCue,
}
