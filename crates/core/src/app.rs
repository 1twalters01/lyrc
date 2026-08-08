use chrono::Duration;
use mpris::{
    client::MprisClient,
    playback::{PlaybackCommand, PlaybackStatus, PlayerEvent},
};
use synchronizer::traits::Synchronizer;

use crate::{
    clock::PlaybackClock,
    renderer::Renderer,
    state::{AppMode, AppState},
};

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

    pub fn select_previous_line(&mut self) {
        match self.state.selected_cue {
            Some(line) => self.state.selected_cue = Some(line.saturating_sub(1)),
            None => {
                self.state.selected_cue = match self.synchronizer.get_active_cues().first() {
                    Some(line_idx) => Some(line_idx.clone()),
                    None => Some(0),
                }
            }
        };
    }

    pub fn select_next_line(&mut self) {
        match self.state.selected_cue {
            Some(line) => {
                self.state.selected_cue = match self.state.subtitle_document {
                    Some(ref document) => {
                        if line < document.cues.len() - 1 {
                            Some(line + 1)
                        } else {
                            Some(line)
                        }
                    }
                    None => None,
                };
            }
            None => {
                self.state.selected_cue = match self.synchronizer.get_active_cues().first() {
                    Some(line_idx) => Some(line_idx.clone()),
                    None => Some(0),
                }
            }
        };
    }

    pub fn clear_line_selection(&mut self) {
        self.state.selected_cue = None;
    }

    pub async fn seek_to_selected_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(selected_cue) = self.state.selected_cue {
            if let Some(ref document) = self.state.subtitle_document {
                let cue = &document.cues[selected_cue];
                let duration = cue.start;
                self.mpris
                    .execute(PlaybackCommand::SetPosition(duration))
                    .await?;
            }
        }

        Ok(())
    }

    pub fn adjust_selected_cue_start_forwards(&mut self, forwards_cue_increment: Duration) {
        if self.state.selected_cue.is_none() {
            self.state.selected_cue = self.synchronizer.get_active_cues().first().copied();
        }

        match (
            &mut self.state.subtitle_document,
            &mut self.state.selected_cue,
            &self.state.track,
        ) {
            (Some(document), Some(line_idx), Some(track)) => {
                let current_cue = &mut document.cues[*line_idx];
                let new_start = current_cue.start + forwards_cue_increment;

                if new_start <= track.duration {
                    current_cue.start = new_start;

                    while *line_idx + 1 < document.cues.len()
                        && &document.cues[*line_idx].start > &document.cues[*line_idx + 1].start
                    {
                        document.cues.swap(*line_idx, *line_idx + 1);
                        *line_idx += 1;
                    }
                }
            }
            _ => {}
        }
    }
    pub fn adjust_selected_cue_start_backwards(&mut self, backwards_cue_increment: Duration) {
        if self.state.selected_cue.is_none() {
            self.state.selected_cue = self.synchronizer.get_active_cues().first().copied();
        }

        match (
            &mut self.state.subtitle_document,
            &mut self.state.selected_cue,
        ) {
            (Some(document), Some(line_idx)) => {
                let current_cue = &mut document.cues[*line_idx];
                let new_start = current_cue.start - backwards_cue_increment;
                if new_start >= Duration::zero() {
                    current_cue.start = new_start;

                    while *line_idx > 0
                        && &document.cues[*line_idx].start < &document.cues[*line_idx - 1].start
                    {
                        document.cues.swap(*line_idx, *line_idx - 1);
                        *line_idx -= 1;
                    }
                }
            }
            _ => {}
        }
    }
    pub fn switch_to_normal_mode(&mut self) {
        self.state.app_mode = AppMode::Normal;
    }
    pub fn switch_to_edit_mode(&mut self) {
        if self.state.selected_cue.is_none() {
            self.state.selected_cue = self.synchronizer.get_active_cues().first().copied();
        }

        if self.state.selected_cue.is_some() {
            self.state.app_mode = AppMode::Edit;
        }
    }
}
