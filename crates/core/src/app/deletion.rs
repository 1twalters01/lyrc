use synchronizer::traits::Synchronizer;

use crate::{app::App, history::IndexedSubtitleCue, mode::AppMode, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub fn delete_selected_lines(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues,
                } => {
                    for index in selected_cues.iter().rev() {
                        subtitle_document.cues.remove(*index);
                        if index < cue_index {
                            *cue_index = cue_index.saturating_sub(1);
                        }
                    }
                    *selected_cues = Vec::new();
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {
                    for cue in selected_cues.iter().rev() {
                        subtitle_document.cues.remove(cue.index);
                        if cue.index < *cue_index {
                            *cue_index = cue_index.saturating_sub(1);
                        }
                    }
                }
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub fn delete_current_line(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues,
                } => {
                    selected_cues.retain(|c_index| c_index != cue_index);
                    subtitle_document.cues.remove(*cue_index);
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {
                    selected_cues.retain(|cue| cue.index != *cue_index);
                    subtitle_document.cues.remove(*cue_index);
                }
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub fn delete_cues(&mut self, mut indexed_cues: Vec<IndexedSubtitleCue>) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        indexed_cues.sort_by_key(|c| c.index);
        match &mut self.state.subtitle_document {
            Some(subtitle_document) => {
                for indexed_cue in indexed_cues.iter().rev() {
                    subtitle_document.cues.remove(indexed_cue.index);

                    match &mut self.state.app_mode {
                        AppMode::Normal => {}
                        AppMode::Select {
                            cue_index: _,
                            selected_cues,
                        } => {
                            selected_cues.retain(|c_index| *c_index != indexed_cue.index);
                            for selected_cue in selected_cues {
                                if *selected_cue > indexed_cue.index {
                                    *selected_cue -= 1;
                                }
                            }
                        }
                        AppMode::Edit {
                            cue_index: _,
                            selected_cues,
                        } => {
                            for selected_cue in selected_cues {
                                if selected_cue.index > indexed_cue.index {
                                    selected_cue.index -= 1;
                                }
                            }
                        }
                    }
                }
            }
            None => self.switch_to_normal_mode(),
        }
    }
}
