use chrono::Duration;

use synchronizer::traits::Synchronizer;

use crate::{app::App, mode::AppMode, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    // change error type
    pub fn increase_selected_cue_start_time(
        &mut self,
        forwards_cue_increment: Duration,
    ) -> Result<(), String> {
        fn increase_cue_index(
            document: &mut subtitles::subtitles::SubtitleDocument,
            cue_index: usize,
            forwards_cue_increment: Duration,
            track: &mpris::track::Track,
        ) -> usize {
            let current_cue = &mut document.cues[cue_index];
            let new_start = current_cue.start + forwards_cue_increment;
            let mut new_index = cue_index;

            if new_start <= track.duration {
                current_cue.start = new_start;

                while new_index + 1 < document.cues.len()
                    && &document.cues[new_index].start > &document.cues[new_index + 1].start
                {
                    document.cues.swap(new_index, new_index + 1);
                    new_index += 1;
                }
            }

            new_index
        }

        match (&mut self.state.subtitle_document, &self.state.track) {
            (Some(document), Some(track)) => {
                match &self.state.app_mode {
                    AppMode::Normal => {
                        return Err(String::from("Cannot be in normal mode"));
                    }
                    AppMode::Select {
                        cue_index,
                        selected_cues,
                    } => AppMode::Select {
                        cue_index: increase_cue_index(
                            document,
                            *cue_index,
                            forwards_cue_increment,
                            track,
                        ),
                        selected_cues: selected_cues.clone(),
                    },
                    AppMode::Edit {
                        cue_index,
                        selected_cues,
                    } => AppMode::Edit {
                        cue_index: increase_cue_index(
                            document,
                            *cue_index,
                            forwards_cue_increment,
                            track,
                        ),
                        selected_cues: selected_cues.clone(),
                    },
                };

                Ok(())
            }
            _ => Err(String::from("No subtitle document found")),
        }
    }

    pub fn decrease_selected_cue_start_time(
        &mut self,
        backwards_cue_increment: Duration,
    ) -> Result<(), String> {
        fn decrease_cue_index(
            document: &mut subtitles::subtitles::SubtitleDocument,
            cue_index: usize,
            backwards_cue_increment: Duration,
        ) -> usize {
            let current_cue = &mut document.cues[cue_index];
            let new_start = current_cue.start - backwards_cue_increment;
            let mut new_index = cue_index;

            if new_start >= Duration::zero() {
                current_cue.start = new_start;

                while new_index > 0
                    && &document.cues[new_index].start < &document.cues[new_index - 1].start
                {
                    document.cues.swap(new_index, new_index - 1);
                    new_index -= 1;
                }
            }

            new_index
        }

        match &mut self.state.subtitle_document {
            Some(document) => {
                match &self.state.app_mode {
                    AppMode::Normal => {
                        return Err(String::from("Cannot be in normal mode"));
                    }
                    AppMode::Select {
                        cue_index,
                        selected_cues,
                    } => AppMode::Select {
                        cue_index: decrease_cue_index(
                            document,
                            *cue_index,
                            backwards_cue_increment,
                        ),
                        selected_cues: selected_cues.clone(),
                    },
                    AppMode::Edit {
                        cue_index,
                        selected_cues,
                    } => AppMode::Edit {
                        cue_index: decrease_cue_index(
                            document,
                            *cue_index,
                            backwards_cue_increment,
                        ),
                        selected_cues: selected_cues.clone(),
                    },
                };

                Ok(())
            }
            _ => Err(String::from("No subtitle document found")),
        }
    }

    pub fn increase_all_cue_start_times(&mut self, forwards_cue_increment: Duration) {
        match (&mut self.state.subtitle_document, &self.state.track) {
            (Some(document), Some(track)) => {
                for cue in &mut document.cues {
                    let new_start = cue.start + forwards_cue_increment;

                    if new_start <= track.duration {
                        cue.start = new_start;
                    }
                }

                document.cues.sort_by_key(|cue| cue.start);
            }

            _ => {}
        }
    }

    pub fn decrease_all_cue_start_times(&mut self, backwards_cue_increment: Duration) {
        match &mut self.state.subtitle_document {
            Some(document) => {
                for cue in &mut document.cues {
                    let new_start = cue.start - backwards_cue_increment;

                    if new_start >= Duration::zero() {
                        cue.start = new_start;
                    }
                }

                document.cues.sort_by_key(|cue| cue.start);
            }

            _ => {}
        }
    }
}
