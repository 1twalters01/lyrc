use synchronizer::traits::Synchronizer;
use crate::{
    app::App,
    history::Edit,
    renderer::Renderer,
};

impl <R, S> App<R, S>
where 
    R: Renderer,
    S: Synchronizer,
{
    pub fn undo(&mut self) {
        if let Some(edit) = self.edit_history.undo.pop() {
            self.undo_edit(&edit);
            self.edit_history.redo.push(edit);
        }
    }

    pub fn redo(&mut self) {
        if let Some(edit) = self.history.redo.pop() {
            self.redo_edit(&edit);
            self.edit_history.undo.push(edit);
        }
    }

    fn undo_edit(&mut self, edit: &Edit) {
        match edit {
            Edit::ChangeContent(
                index,
                old_content,
                new_content,
            ) => self.subtitle_document.cues[index].content = old_content,
        }
    }

    fn redo_edit(&mut self, edit: &Edit) {
        match edit {
            Edit::ChangeContent(
                index,
                old_content,
                new_content,
            ) => self.subtitle_document.cues[index].content = new_content,
        }
    }
}