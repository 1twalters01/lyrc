use std::time::Duration;

use crate::{keyboard, workers::start::Workers};

use configuration::config::Config;
use lyrc_core::app::App;
use tokio::time::interval;
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;

pub async fn handle_tui_events(
    mut app: App<TuiRenderer>,
    mut workers: Workers,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mpris = app.mpris.clone();
    let mut events = mpris.events().await?;
    let mut tick = interval(Duration::from_secs_f64(1f64 / config.fps));
    let mut keyboard = EventStream::new();

    while !app.state.quit {
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
