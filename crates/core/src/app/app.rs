use alignment::messages::AlignmentRequest;
use chrono::Duration;
use mpris::client::MprisClient;
use synchronizer::traits::Synchronizer;
use tokio::sync::mpsc::Sender;

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
    pub alignment_req_tx: Sender<AlignmentRequest>,
}

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub async fn new(
        renderer: R,
        synchronizer: S,
        clock_offset: Duration,
        player: &str,
        alignment_req_tx: Sender<AlignmentRequest>,
    ) -> Self {
        let clock = PlaybackClock::new(clock_offset);
        let state = AppState::new();
        let mpris = MprisClient::connect(player).await.unwrap();

        Self {
            renderer,
            clock,
            state,
            synchronizer,
            mpris,
            alignment_req_tx,
        }
    }
}
