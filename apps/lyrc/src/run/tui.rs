use std::time::Duration as std_duration;

use crate::{config::Config, keyboard, workers::start::Workers};

use lyrc_core::app::App;
use mpris::client::MprisClient;
use synchronizer::strategies::{cues::CueSynchronizer, words::WordSynchronizer};
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;

pub async fn run_tui(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut workers = Workers::start().await;

    let player = &MprisClient::choose_player(&config.targets_in_priority_order).await?;

    let cue_synchronizer = CueSynchronizer::new();
    let word_synchronizer = WordSynchronizer::new();

    let mut app = App::new(
        TuiRenderer::new()?,
        cue_synchronizer,
        word_synchronizer,
        config.clock_offset,
        player,
        workers.alignment.request_tx,
        workers.translation.request_tx,
    )
    .await;

    app.update_track().await;
    app.update_subtitle_document().await;

    let mpris = app.mpris.clone();
    let mut events = mpris.events().await?;
    let mut tick = tokio::time::interval(std_duration::from_secs_f64(1f64 / config.fps));
    let mut keyboard = EventStream::new();

    while app.state.quit == false {
        tokio::select! {
            Some(event) = events.next() => app.handle_player_event(event).await?,

            _ = tick.tick() => app.handle_tick_event().await?,

            Some(Ok(Event::Key(key))) = keyboard.next() => keyboard::handle_keyboard_event(&mut app, key, &config).await?,

            Some(result) = workers.alignment.result_rx.recv() => app.handle_alignment_event(result)?,

            Some(result) = workers.translation.result_rx.recv() => app.handle_translation_event(result)?,
        }
    }

    Ok(())
}
