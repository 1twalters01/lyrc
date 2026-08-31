use std::time::Duration as std_duration;

use crate::{
    args::{Args, Command, Frontend},
    keyboard,
    workers::{alignment::start_alignment_worker, translation::start_translation_worker},
};

use alignment::messages::{AlignmentRequest, AlignmentResult};
use lyrc_core::app::App;
use mpris::client::MprisClient;
use synchronizer::strategies::{cues::CueSynchronizer, words::WordSynchronizer};
use translation::messages::{TranslationRequest, TranslationResult};
use tui::renderer::TuiRenderer;

use chrono::Duration;
use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    let targets_in_priority_order = Vec::from(["mpv", "cmus"]);
    let player = &MprisClient::choose_player(targets_in_priority_order).await?;

    let fps = 60f64;
    let clock_offset = Duration::milliseconds(0);
    let rewind_duration = Duration::milliseconds(-5000);
    let fast_forward_duration = Duration::milliseconds(5000);
    let forwards_cue_increment_small = Duration::milliseconds(10);
    let backwards_cue_increment_small = Duration::milliseconds(10);
    let forwards_cue_increment_large = Duration::milliseconds(500);
    let backwards_cue_increment_large = Duration::milliseconds(500);

    let config = crate::config::Config {
        rewind_duration,
        fast_forward_duration,
        forwards_cue_increment_small,
        backwards_cue_increment_small,
        forwards_cue_increment_large,
        backwards_cue_increment_large,
    };

    match command {
        Command::Daemon => {
            println!("daemon");
            Ok(())
        }
        Command::App { frontend } => {
            let cue_synchronizer = CueSynchronizer::new();
            let word_synchronizer = WordSynchronizer::new();

            let renderer = match frontend {
                Frontend::Tui => TuiRenderer::new().unwrap(),
            };

            let (alignment_req_tx, alignment_req_rx) =
                tokio::sync::mpsc::channel::<AlignmentRequest>(1);
            let (alignment_res_tx, mut alignment_res_rx) =
                tokio::sync::mpsc::channel::<AlignmentResult>(1);
            let (translation_req_tx, translation_req_rx) =
                tokio::sync::mpsc::channel::<TranslationRequest>(1);
            let (translation_res_tx, mut translation_res_rx) =
                tokio::sync::mpsc::channel::<TranslationResult>(1);

            let mut app = App::new(
                renderer,
                cue_synchronizer,
                word_synchronizer,
                clock_offset,
                player,
                alignment_req_tx,
                translation_req_tx,
            )
            .await;
            app.update_track_and_subtitle_document_information().await;

            start_alignment_worker(alignment_req_rx, alignment_res_tx);
            start_translation_worker(translation_req_rx, translation_res_tx);

            let mpris = app.mpris.clone();
            let mut events = mpris.events().await?;
            let mut tick = tokio::time::interval(std_duration::from_secs_f64(1f64 / fps));
            let mut keyboard = EventStream::new();

            loop {
                tokio::select! {
                    Some(event) = events.next() => app.handle_player_event(event).await?,

                    _ = tick.tick() => app.handle_tick_event().await?,

                    Some(Ok(Event::Key(key))) = keyboard.next() => keyboard::handle_keyboard_event(&mut app, key, &config).await?,

                    Some(result) = alignment_res_rx.recv() => app.handle_alignment_event(result)?,

                    Some(result) = translation_res_rx.recv() => app.handle_translation_event(result)?,
                }

                if app.state.quit == true {
                    break Ok(());
                }
            }
        }
    }
}
