use std::time::Duration as std_duration;

use crate::{
    args::{Args, Command, Frontend},
    config::Config,
    keyboard,
    workers::{alignment::start_alignment_worker, translation::start_translation_worker},
};

use alignment::messages::{AlignmentRequest, AlignmentResult};
use gui::renderer::GuiRenderer;
use lyrc_core::app::App;
use mpris::client::MprisClient;
use synchronizer::strategies::{cues::CueSynchronizer, words::WordSynchronizer};
use translation::messages::{TranslationRequest, TranslationResult};
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;

pub async fn run_gui(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let player = &MprisClient::choose_player(&config.targets_in_priority_order).await?;

    let cue_synchronizer = CueSynchronizer::new();
    let word_synchronizer = WordSynchronizer::new();

    let (alignment_req_tx, alignment_req_rx) = tokio::sync::mpsc::channel::<AlignmentRequest>(1);
    let (alignment_res_tx, mut alignment_res_rx) = tokio::sync::mpsc::channel::<AlignmentResult>(1);
    start_alignment_worker(alignment_req_rx, alignment_res_tx);

    let (translation_req_tx, translation_req_rx) =
        tokio::sync::mpsc::channel::<TranslationRequest>(1);
    let (translation_res_tx, mut translation_res_rx) =
        tokio::sync::mpsc::channel::<TranslationResult>(1);
    start_translation_worker(translation_req_rx, translation_res_tx).await;

    let mut app = App::new(
        GuiRenderer::new()?,
        cue_synchronizer,
        word_synchronizer,
        config.clock_offset,
        player,
        alignment_req_tx,
        translation_req_tx,
    )
    .await;

    app.update_track_and_subtitle_document_information().await;

    let mpris = app.mpris.clone();
    let mut events = mpris.events().await?;

    todo!()
}
