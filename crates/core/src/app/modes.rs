use std::fs;

use chrono::Duration;
use subtitles::subtitles::SubtitleDocument;
use synchronizer::traits::Synchronizer;

use crate::{
    app::App,
    mode::{AppMode, EditCue},
    renderer::Renderer,
};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub fn switch_to_normal_mode(&mut self) {
        self.state.app_mode = AppMode::Normal;
    }

    pub fn switch_to_select_mode(&mut self) -> Result<(), String> {
        if self.state.subtitle_document.is_none() {
            return Err(String::from("No subtitle document found"));
        }

        let (cue_index, selected_cues) = match &self.state.app_mode {
            AppMode::Normal => (
                *self
                    .synchronizer
                    .get_active_cue_indices()
                    .first()
                    .unwrap_or(&0),
                Vec::new(),
            ),
            AppMode::Select {
                cue_index,
                selected_cues,
            } => (*cue_index, selected_cues.clone()),
            AppMode::Edit {
                cue_index,
                selected_cues,
            } => (
                *cue_index,
                selected_cues
                    .iter()
                    .map(|edit_cue| edit_cue.index)
                    .collect(),
            ),
        };

        self.state.app_mode = AppMode::Select {
            cue_index,
            selected_cues,
        };

        Ok(())
    }

    pub fn switch_to_edit_mode(&mut self) -> Result<(), String> {
        let subtitle_document = match &self.state.subtitle_document {
            Some(subtitle_document) => subtitle_document,
            None => return Err(String::from("No subtitle document found")),
        };

        let (cue_index, selected_cues) = match &self.state.app_mode {
            AppMode::Normal => {
                let index = *self
                    .synchronizer
                    .get_active_cue_indices()
                    .first()
                    .unwrap_or(&0);
                let original_content = subtitle_document.cues[index].content.clone();
                let selected_edit_cues = Vec::from([EditCue {
                    index,
                    original_content,
                }]);
                (index, selected_edit_cues.clone())
            }
            AppMode::Select {
                cue_index,
                selected_cues,
            } => {
                let selected_edit_cues = selected_cues
                    .iter()
                    .map(|index| EditCue {
                        index: *index,
                        original_content: subtitle_document.cues[*index].content.clone(),
                    })
                    .collect::<Vec<EditCue>>();
                (*cue_index, selected_edit_cues.clone())
            }

            AppMode::Edit {
                cue_index,
                selected_cues,
            } => (*cue_index, selected_cues.clone()),
        };

        self.state.app_mode = AppMode::Edit {
            cue_index,
            selected_cues,
        };

        Ok(())
    }
}
