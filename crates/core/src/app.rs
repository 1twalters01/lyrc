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
}
