use subtitles::subtitles::SubtitleContent;

#[derive(Clone)]
pub struct EditHistory {
    pub undo: Vec<Edit>,
    pub redo: Vec<Edit>,
}

#[derive(Clone)]
pub enum Edit {
    ChangeContent {
        index: usize,
        old_content: SubtitleContent,
        new_content: SubtitleContent,
    },
    // Delete cues
    // Insert cues
    // Change Times
}
