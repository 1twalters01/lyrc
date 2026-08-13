use std::fs;

use chrono::Duration;
use mpris::{
    client::MprisClient,
    playback::{PlaybackCommand, PlaybackStatus, PlayerEvent},
};
use subtitles::subtitles::SubtitleDocument;
use synchronizer::traits::Synchronizer;

use crate::{clock::PlaybackClock, mode::AppMode, renderer::Renderer, state::AppState};

pub struct App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    renderer: R,
    pub synchronizer: S,
    pub clock: PlaybackClock,
    pub state: AppState,
    pub mpris: MprisClient,
}

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub async fn new(renderer: R, synchronizer: S, clock_offset: Duration, player: &str) -> Self {
        let clock = PlaybackClock::new(clock_offset);
        let state = AppState::new();
        let mpris = MprisClient::connect(player).await.unwrap();

        Self {
            renderer,
            clock,
            state,
            synchronizer,
            mpris,
        }
    }

    pub async fn handle_player_event(
        &mut self,
        event: PlayerEvent,
    ) -> Result<(), <R as Renderer>::Error> {
        self.state.update(&event);
        self.synchronizer
            .update(&self.state.subtitle_document, &self.clock.get_position());
        self.clock.update(event);

        self.renderer.render(
            &mut self.state,
            self.clock.get_position(),
            self.synchronizer.get_active_cues(),
        )
    }

    pub async fn tick(&mut self) -> Result<(), <R as Renderer>::Error> {
        let current_position = self.get_current_position().await;
        let playback_status = self.get_playback_status().await;
        self.handle_tick(current_position, playback_status)
    }

    pub fn handle_tick(
        &mut self,
        current_position: Option<Duration>,
        playback_status: PlaybackStatus,
    ) -> Result<(), <R as Renderer>::Error> {
        self.synchronizer
            .update(&self.state.subtitle_document, &self.clock.get_position());
        self.clock.sync(current_position, playback_status).unwrap();
        self.renderer.render(
            &mut self.state,
            self.clock.get_position(),
            self.synchronizer.get_active_cues(),
        )
    }

    pub async fn get_current_position(&self) -> Option<Duration> {
        self.mpris.get_current_position().await.ok()
    }

    pub async fn get_playback_status(&self) -> PlaybackStatus {
        self.mpris
            .get_playback_status()
            .await
            .unwrap_or(PlaybackStatus::Unknown)
    }

    pub async fn toggle_play_pause(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.mpris.execute(PlaybackCommand::Toggle).await?;

        Ok(())
    }

    pub async fn seek_by_duration(
        &mut self,
        duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.mpris.execute(PlaybackCommand::Seek(duration)).await?;

        Ok(())
    }

    pub fn get_first_active_cue(&self) -> Option<usize> {
        match &self.state.subtitle_document {
            Some(_document) => match self.synchronizer.get_active_cues().first() {
                Some(line_idx) => Some(line_idx.clone()),
                None => Some(0),
            },
            None => None,
        }
    }

    // rename to go_to_previous_line
    // remove cue_index parameter?
    pub fn select_previous_line(&mut self, cue_index: usize) {
        if self.state.track.is_none() || self.state.subtitle_document.is_none() {
            self.switch_to_normal_mode();
        }

        let selected_cues = match &self.state.app_mode {
            AppMode::Normal => Vec::new(),
            AppMode::Select {
                cue_index,
                selected_cues,
            } => selected_cues.clone(),
            AppMode::Edit {
                cue_index,
                original_content,
            } => Vec::new(),
        };

        self.state.app_mode = AppMode::Select {
            cue_index: cue_index.saturating_sub(1),
            selected_cues,
        };
    }

    // rename to go_to_next_line
    // remove cue_index parameter?
    pub fn select_next_line(&mut self, cue_index: usize) {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        let selected_cues = match &self.state.app_mode {
            AppMode::Normal => Vec::new(),
            AppMode::Select {
                cue_index,
                selected_cues,
            } => selected_cues.clone(),
            AppMode::Edit {
                cue_index,
                original_content,
            } => Vec::new(),
        };

        match &self.state.subtitle_document {
            Some(subtitle_document) => {
                if cue_index < subtitle_document.cues.len() - 1 {
                    self.state.app_mode = AppMode::Select {
                        cue_index: cue_index + 1,
                        selected_cues,
                    }
                }
            }
            None => self.switch_to_normal_mode(),
        }
    }

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
                    original_content,
                } => {}
            },
            None => self.switch_to_normal_mode(),
        }
    }

    pub async fn seek_to_selected_line(
        &mut self,
        cue_index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.track.is_none() {
            self.switch_to_normal_mode();
        }

        if let Some(ref document) = self.state.subtitle_document {
            let cue = &document.cues[cue_index];
            let duration = cue.start;
            self.mpris
                .execute(PlaybackCommand::SetPosition(duration))
                .await?;
        }

        Ok(())
    }

    pub fn adjust_selected_cue_start_forwards(
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
                    AppMode::Normal => return Err(String::from("Cannot be in normal mode")),
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
                        original_content,
                    } => AppMode::Edit {
                        cue_index: increase_cue_index(
                            document,
                            *cue_index,
                            forwards_cue_increment,
                            track,
                        ),
                        original_content: original_content.clone(),
                    },
                };

                Ok(())
            }
            _ => Err(String::from("No subtitle document found")),
        }
    }

    pub fn adjust_selected_cue_start_backwards(
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
                    AppMode::Normal => return Err(String::from("Cannot be in normal mode")),
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
                        original_content,
                    } => AppMode::Edit {
                        cue_index: decrease_cue_index(
                            document,
                            *cue_index,
                            backwards_cue_increment,
                        ),
                        original_content: original_content.clone(),
                    },
                };

                Ok(())
            }
            _ => Err(String::from("No subtitle document found")),
        }
    }

    pub fn adjust_all_cues_start_forwards(&mut self, forwards_cue_increment: Duration) {
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

    pub fn adjust_all_cues_start_backwards(&mut self, backwards_cue_increment: Duration) {
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

    pub fn switch_to_normal_mode(&mut self) {
        self.state.app_mode = AppMode::Normal;
    }

    pub fn switch_to_select_mode(&mut self) -> Result<(), String> {
        if self.state.subtitle_document.is_none() {
            return Err(String::from("No subtitle document found"));
        }

        let (cue_index, selected_cues) = match &self.state.app_mode {
            AppMode::Normal => (
                *self.synchronizer.get_active_cues().first().unwrap_or(&0),
                Vec::new(),
            ),
            AppMode::Select {
                cue_index,
                selected_cues,
            } => (*cue_index, selected_cues.clone()),
            AppMode::Edit {
                cue_index,
                original_content: _,
            } => (*cue_index, Vec::new()),
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

        let (cue_index, original_content) = match &self.state.app_mode {
            AppMode::Normal => {
                let cue_index = *self.synchronizer.get_active_cues().first().unwrap_or(&0);
                let original_content = &subtitle_document.cues[cue_index].content;
                (cue_index, original_content.clone())
            }
            AppMode::Select {
                cue_index,
                selected_cues,
            } => {
                let original_content = &subtitle_document.cues[*cue_index].content;
                (*cue_index, original_content.clone())
            }

            AppMode::Edit {
                cue_index,
                original_content,
            } => (*cue_index, original_content.clone()),
        };

        self.state.app_mode = AppMode::Edit {
            cue_index,
            original_content,
        };

        Ok(())
    }

    pub fn save_document(
        &mut self,
        document: SubtitleDocument,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &document.metadata.file_path {
            Some(file_path) => {
                let file = SubtitleDocument::write(document.clone())?;
                Ok(fs::write(file_path, file)?)
            }
            None => Ok(()),
        }
    }
}
