use alignment::messages::AlignmentRequest;
use chrono::Duration;
use configuration::config::Config;
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
        player: &str,
        alignment_req_tx: Sender<AlignmentRequest>,
        translation_req_tx: Sender<TranslationRequest>,
        config: &Config,
    ) -> Self {
        let cue_synchronizer = CueSynchronizer::new();
        let word_synchronizer = WordSynchronizer::new();

        let clock = PlaybackClock::new(config.clock_offset);
        let state = AppState::new();
        let mpris = MprisClient::connect(player).await.unwrap();
        let synchronizer = AppSynchronizer {
            cue_synchronizer,
            word_synchronizer,
            mode: SynchronizerMode::Cue,
        };

        let mut app = Self {
            renderer,
            clock,
            state,
            synchronizer,
            mpris,
            alignment_req_tx,
            translation_req_tx,
        };

        app.update_track().await;
        app.update_subtitle_document().await;

        app
    }
}
