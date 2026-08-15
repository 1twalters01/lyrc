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
                    selected_cues,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[*cue_index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };
                        subtitle_document
                            .cues
                            .insert(*cue_index + 1, empty_subtitle);

                        for cue in selected_cues.iter_mut() {
                            if *cue + 1 >= *cue_index {
                                *cue += 1;
                            }
                        }
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[*cue_index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };
                        subtitle_document
                            .cues
                            .insert(*cue_index + 1, empty_subtitle);

                        for cue in selected_cues.iter_mut() {
                            if cue.index + 1 >= *cue_index {
                                cue.index += 1;
                            }
                        }
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
                    selected_cues,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[*cue_index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };
                        subtitle_document.cues.insert(*cue_index, empty_subtitle);

                        for cue in selected_cues.iter_mut() {
                            if cue >= cue_index {
                                *cue += 1;
                            }
                        }
                        *cue_index += 1;
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {
                    if subtitle_document.cues.len() > *cue_index {
                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[*cue_index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };
                        subtitle_document.cues.insert(*cue_index, empty_subtitle);

                        for cue in selected_cues.iter_mut() {
                            if cue.index >= *cue_index {
                                cue.index += 1;
                            }
                        }
                        *cue_index += 1;
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
                } => {
                    for i in 0..selected_cues.len() {
                        let index = selected_cues[i] + i;

                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };

                        subtitle_document.cues.insert(index, empty_subtitle);

                        selected_cues[i] += i + 1;
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {
                    for i in 0..selected_cues.len() {
                        let index = selected_cues[i].index + i;

                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };

                        subtitle_document.cues.insert(index, empty_subtitle);

                        selected_cues[i].index += i + 1;
                    }
                }
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
                } => {
                    for i in 0..selected_cues.len() {
                        let index = selected_cues[i] + i;

                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };

                        subtitle_document.cues.insert(index, empty_subtitle);

                        selected_cues[i] += i + 1;
                    }
                }
                AppMode::Edit {
                    cue_index,
                    selected_cues,
                } => {
                    for i in 0..selected_cues.len() {
                        let index = selected_cues[i].index + i;

                        let empty_subtitle = SubtitleCue {
                            id: None,
                            start: subtitle_document.cues[index].start,
                            end: None,
                            content: SubtitleContent::Text(String::new()),
                        };

                        subtitle_document.cues.insert(index, empty_subtitle);

                        selected_cues[i].index += i + 1;
                    }
                }
            },
            None => self.switch_to_normal_mode(),
        }
    }
}
