use chrono::Duration;
use mpris::{
    client::MprisClient,
    playback::{PlaybackStatus, PlayerEvent},
};
use synchronizer::traits::Synchronizer;

use crate::{clock::PlaybackClock, renderer::Renderer, state::AppState};

pub struct App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    renderer: R,
    synchronizer: S,
    clock: PlaybackClock,
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

    pub fn handle_player_event(
        &mut self,
        event: PlayerEvent,
    ) -> Result<(), <R as Renderer>::Error> {
        self.state.update(&event);
        self.update_subtitles();
        self.clock.update(event);
        self.synchronizer.update(
            self.state.subtitle_document,
            self.get_current_position()
        );

        self.renderer.render(
            &self.state,
            self.clock.get_position(),
            self.synchronizer.get_active_cues(),
        )
    }

    pub fn handle_tick(
        &mut self,
        current_position: Option<Duration>,
        playback_status: PlaybackStatus,
    ) -> Result<(), <R as Renderer>::Error> {
        self.update_subtitles();
        if let Some(position) = current_position {
            self.clock.sync(position, playback_status).unwrap();
        }
        self.renderer.render(
            &self.state,
            self.clock.get_position(),
            self.synchronizer.get_active_cues(),
        )
    }

    fn update_subtitles(&mut self) {
        let subtitle_document = &self.state.subtitle_document;
        if let Some(subtitles) = subtitle_document {
            let position = &self.clock.get_position();
            self.synchronizer.update(subtitles, position);
        }
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
}
