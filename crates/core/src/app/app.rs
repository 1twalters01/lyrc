use alignment::messages::AlignmentRequest;
use chrono::Duration;
use mpris::client::MprisClient;
use synchronizer::{
    strategies::{
        cues::{CueIndex, CueSynchronizer},
        words::{WordIndex, WordSynchronizer},
    },
    traits::Synchronizer,
};
use tokio::sync::mpsc::Sender;
use translation::messages::TranslationRequest;

use crate::{
    clock::PlaybackClock,
    renderer::Renderer,
    state::AppState,
    synchronizer::{AppSynchronizer, SynchronizerMode},
};

pub struct App<R>
where
    R: Renderer,
{
    pub renderer: R,
    pub synchronizer: AppSynchronizer,
    pub clock: PlaybackClock,
    pub state: AppState,
    pub mpris: MprisClient,
    pub alignment_req_tx: Sender<AlignmentRequest>,
    pub translation_req_tx: Sender<TranslationRequest>,
}

impl<R> App<R>
where
    R: Renderer,
{
    pub async fn new(
        renderer: R,
        cue_synchronizer: CueSynchronizer,
        word_synchronizer: WordSynchronizer,
        clock_offset: Duration,
        player: &str,
        alignment_req_tx: Sender<AlignmentRequest>,
        translation_req_tx: Sender<TranslationRequest>,
    ) -> Self {
        let clock = PlaybackClock::new(clock_offset);
        let state = AppState::new();
        let mpris = MprisClient::connect(player).await.unwrap();
        let synchronizer = AppSynchronizer {
            cue_synchronizer,
            word_synchronizer,
            mode: SynchronizerMode::Cue,
        };

        Self {
            renderer,
            clock,
            state,
            synchronizer,
            mpris,
            alignment_req_tx,
            translation_req_tx,
        }
    }
}
