use subtitles::subtitles::SubtitleContent;

pub struct EditHistory {
    undo: Vec<Edit>,
    redo: Vec<Edit>,
}

pub enum Edit {
    ChangeContent {
        index: usize,
        old_content: SubtitleContent,
        new_content: SubtitleContent,
    },
}