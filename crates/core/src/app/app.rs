use chrono::Duration;
use mpris::client::MprisClient;
use synchronizer::traits::Synchronizer;

use crate::{clock::PlaybackClock, renderer::Renderer, state::AppState};

pub struct App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub renderer: R,
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
}
