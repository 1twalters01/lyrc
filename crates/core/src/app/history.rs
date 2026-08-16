use crate::{app::App, history::Edit, renderer::Renderer};
use synchronizer::traits::Synchronizer;

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub fn push_to_history(&mut self, edit: Edit) {
        self.state.edit_history.undo.push(edit);
        self.state.edit_history.redo.clear();
    }

    pub fn undo(&mut self) {
        if let Some(edit) = self.state.edit_history.undo.pop() {
            self.undo_edit(&edit);
            self.state.edit_history.redo.push(edit);
        }
    }

    pub fn redo(&mut self) {
        if let Some(edit) = self.state.edit_history.redo.pop() {
            self.redo_edit(&edit);
            self.state.edit_history.undo.push(edit);
        }
    }

    fn undo_edit(&mut self, edit: &Edit) {
        match edit {
            Edit::ChangeContent {
                index,
                old_content,
                new_content: _,
            } => match &mut self.state.subtitle_document {
                Some(subtitle_document) => {
                    subtitle_document.cues[*index].content = old_content.clone()
                }
                None => {}
            },
        }
    }

    fn redo_edit(&mut self, edit: &Edit) {
        match edit {
            Edit::ChangeContent {
                index,
                old_content: _,
                new_content,
            } => match &mut self.state.subtitle_document {
                Some(subtitle_document) => {
                    subtitle_document.cues[*index].content = new_content.clone()
                }
                None => {}
            },
        }
    }
}
