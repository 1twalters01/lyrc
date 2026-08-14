use synchronizer::traits::Synchronizer;

use crate::{app::App, mode::AppMode, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub fn toggle_select_line(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &self.state.subtitle_document {
            Some(_) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues,
                } => {
                    if selected_cues.contains(&cue_index) {
                        let index = selected_cues.iter().position(|c| c == cue_index).unwrap();
                        selected_cues.remove(index);
                    } else {
                        selected_cues.push(*cue_index);
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {}
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub fn toggle_select_all_lines(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
            return Err(String::from("No track found").into());
        }

        self.switch_to_select_mode()?;

        match &self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues,
                } => {
                    if selected_cues.is_empty() {
                        *selected_cues = (0..subtitle_document.cues.len()).collect::<Vec<usize>>();
                    } else {
                        *selected_cues = Vec::new();
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {}
            },
            None => {
                self.switch_to_normal_mode();
                return Err(String::from("No subtitle document found").into());
            }
        }

        Ok(())
    }
}
