use subtitles::subtitles::{SubtitleContent, SubtitleCue};
use synchronizer::traits::Synchronizer;

use crate::{app::App, mode::AppMode, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub fn add_cue_after_current_cue(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues: _,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let start = subtitle_document.cues[*cue_index].start;
                        let content = SubtitleContent::Text(String::new());
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start,
                            end: None,
                            content,
                        };
                        subtitle_document
                            .cues
                            .insert(*cue_index + 1, empty_subtitle);
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues: _,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let start = subtitle_document.cues[*cue_index].start;
                        let content = SubtitleContent::Text(String::new());
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start,
                            end: None,
                            content,
                        };
                        subtitle_document
                            .cues
                            .insert(*cue_index + 1, empty_subtitle);
                    }
                }
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub fn add_cue_before_current_cue(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues: _,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let start = subtitle_document.cues[*cue_index].start;
                        let content = SubtitleContent::Text(String::new());
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start,
                            end: None,
                            content,
                        };
                        subtitle_document.cues.insert(*cue_index, empty_subtitle);
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues: _,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let start = subtitle_document.cues[*cue_index].start;
                        let content = SubtitleContent::Text(String::new());
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start,
                            end: None,
                            content,
                        };
                        subtitle_document.cues.insert(*cue_index, empty_subtitle);
                    }
                }
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub fn add_cue_after_selected_cues(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues,
                } => {}
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {}
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub fn add_cue_before_selected_cues(&mut self) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        match &mut self.state.subtitle_document {
            Some(subtitle_document) => match &mut self.state.app_mode {
                AppMode::Normal => {}
                AppMode::Select {
                    cue_index,
                    selected_cues,
                } => {}
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {}
            },
            None => self.switch_to_normal_mode(),
        }
    }
}
