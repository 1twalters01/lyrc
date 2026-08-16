use crate::{app::App, history::Edit, renderer::Renderer};
use synchronizer::traits::Synchronizer;

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub fn push_to_history(&mut self, edit: Edit) {
        self.state.edit_history.push(edit);
    }

    pub fn undo(&mut self) {
        if let Some(edit) = self.state.edit_history.pop_undo() {
            self.undo_edit(&edit);
            self.state.edit_history.push_redo(edit);
        }

        if self.state.edit_history.is_empty() {
            self.state.unsaved_changes = false;
        }
    }

    pub fn redo(&mut self) {
        if let Some(edit) = self.state.edit_history.pop_redo() {
            self.state.unsaved_changes = true;
            self.redo_edit(&edit);
            self.state.edit_history.push_undo(edit);
        }
    }

    fn undo_edit(&mut self, edit: &Edit) {
        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match edit {
                Edit::EditCueContent { changes } => {
                    for change in changes {
                        subtitle_document.cues[change.index].content = change.old_content.clone()
                    }
                }
                Edit::EditCueTimes { changes } => for change in changes {},
                Edit::DeleteCue { cues } => {
                    self.insert_cues(cues.clone());
                }
                Edit::InsertCue { cues } => {
                    self.delete_cues(cues.clone());
                }
            },
            None => {}
        }
    }

    fn redo_edit(&mut self, edit: &Edit) {
        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match edit {
                Edit::EditCueContent { changes } => {
                    for change in changes {
                        subtitle_document.cues[change.index].content = change.new_content.clone()
                    }
                }
                Edit::EditCueTimes { changes } => for change in changes {},
                Edit::DeleteCue { cues } => {
                    self.delete_cues(cues.clone());
                }
                Edit::InsertCue { cues } => {
                    self.insert_cues(cues.clone());
                }
            },
            None => {}
        }
    }
}
