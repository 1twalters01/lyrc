use synchronizer::traits::{CueIndexed, Synchronizer};

use crate::{app::App, mode::AppMode, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer<S::Active>,
    S: Synchronizer,
{
    pub fn get_first_active_index(&self) -> Option<S::Active> {
        match &self.state.subtitle_document {
            Some(_document) => Some(
                self.synchronizer
                    .get_active_indices()
                    .first()
                    .cloned()
                    .unwrap_or_default(),
            ),
            None => None,
        }
    }

    pub fn get_first_active_cue_index(&self) -> Option<usize> {
        match &self.state.subtitle_document {
            Some(_document) => match self
                .synchronizer
                .get_active_indices()
                .iter()
                .map(|i| i.cue_index().cue)
                .collect::<Vec<_>>()
                .first()
            {
                Some(line_idx) => Some(line_idx.clone()),
                None => Some(0),
            },
            None => None,
        }
    }

    // Edit mode should go to previous edit line rather than previous line
    pub fn go_to_previous_line(&mut self) {
        if self.state.track.is_none() || self.state.subtitle_document.is_none() {
            self.switch_to_normal_mode();
        }

        let app_mode = match &self.state.app_mode {
            AppMode::Normal => match self.get_first_active_cue_index() {
                Some(index) => AppMode::Select {
                    cue_index: index,
                    selected_cues: Vec::new(),
                },
                None => AppMode::Normal,
            },
            AppMode::Select {
                cue_index,
                selected_cues,
            } => AppMode::Select {
                cue_index: cue_index.saturating_sub(1),
                selected_cues: selected_cues.clone(),
            },
            AppMode::Edit {
                cue_index,
                selected_cues,
            } => AppMode::Edit {
                cue_index: cue_index.saturating_sub(1),
                selected_cues: selected_cues.clone(),
            },
        };

        self.state.app_mode = app_mode;
    }

    // Edit mode should go to next edit line rather than next line
    pub fn go_to_next_line(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        let app_mode = match &self.state.app_mode {
            AppMode::Normal => match self.get_first_active_cue_index() {
                Some(index) => AppMode::Select {
                    cue_index: index,
                    selected_cues: Vec::new(),
                },
                None => AppMode::Normal,
            },
            AppMode::Select {
                cue_index,
                selected_cues,
            } => match &self.state.subtitle_document {
                Some(subtitle_document) => {
                    let mut index = *cue_index;
                    if index < subtitle_document.cues.len() - 1 {
                        index += 1;
                    }
                    AppMode::Select {
                        cue_index: index,
                        selected_cues: selected_cues.clone(),
                    }
                }
                None => AppMode::Normal,
            },
            AppMode::Edit {
                cue_index,
                selected_cues,
            } => match &self.state.subtitle_document {
                Some(subtitle_document) => {
                    let mut index = *cue_index;
                    if index < subtitle_document.cues.len() - 1 {
                        index += 1;
                    }
                    AppMode::Edit {
                        cue_index: index,
                        selected_cues: selected_cues.clone(),
                    }
                }
                None => AppMode::Normal,
            },
        };

        self.state.app_mode = app_mode;
    }

    pub fn go_to_previous_half_page(&mut self) {
        if self.state.track.is_none() || self.state.subtitle_document.is_none() {
            self.switch_to_normal_mode();
        }

        let lines = self.renderer.get_lines_per_page() / 2;

        let app_mode = match &self.state.app_mode {
            AppMode::Normal => match self.get_first_active_cue_index() {
                Some(index) => AppMode::Select {
                    cue_index: index,
                    selected_cues: Vec::new(),
                },
                None => AppMode::Normal,
            },
            AppMode::Select {
                cue_index,
                selected_cues,
            } => AppMode::Select {
                cue_index: cue_index.saturating_sub(lines),
                selected_cues: selected_cues.clone(),
            },
            AppMode::Edit {
                cue_index,
                selected_cues,
            } => AppMode::Edit {
                cue_index: cue_index.saturating_sub(lines),
                selected_cues: selected_cues.clone(),
            },
        };

        self.state.app_mode = app_mode;
    }

    pub fn go_to_next_half_page(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        let lines = self.renderer.get_lines_per_page() / 2;
        let app_mode = match &self.state.app_mode {
            AppMode::Normal => match self.get_first_active_cue_index() {
                Some(index) => AppMode::Select {
                    cue_index: index,
                    selected_cues: Vec::new(),
                },
                None => AppMode::Normal,
            },
            AppMode::Select {
                cue_index,
                selected_cues,
            } => match &self.state.subtitle_document {
                Some(subtitle_document) => {
                    let mut index = *cue_index;
                    if index < subtitle_document.cues.len() - lines {
                        index += lines;
                    } else {
                        index = subtitle_document.cues.len() - 1;
                    }

                    AppMode::Select {
                        cue_index: index,
                        selected_cues: selected_cues.clone(),
                    }
                }
                None => AppMode::Normal,
            },
            AppMode::Edit {
                cue_index,
                selected_cues,
            } => match &self.state.subtitle_document {
                Some(subtitle_document) => {
                    let mut index = *cue_index;
                    if index < subtitle_document.cues.len() - lines {
                        index += lines;
                    } else {
                        index = subtitle_document.cues.len() - 1;
                    }

                    AppMode::Edit {
                        cue_index: index,
                        selected_cues: selected_cues.clone(),
                    }
                }
                None => AppMode::Normal,
            },
        };

        self.state.app_mode = app_mode;
    }
}
