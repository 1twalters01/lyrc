use std::time::Duration as std_duration;

use crate::{
    args::{Args, Command, Frontend},
    keyboard,
};

use lyrc_core::app::App;
use synchronizer::strategies::lyrics::LyricsSynchronizer;
use tui::renderer::TuiRenderer;

use chrono::Duration;
use crossterm::event::{Event, EventStream};
use futures_util::stream::StreamExt;

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    // println!("Initialisation code");
    let fps = 60f64;
    let player = "cmus"; // also want to be able to automatically find a player
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
            let synchronizer = LyricsSynchronizer::new();

            let renderer = match frontend {
                Frontend::Tui => TuiRenderer::new().unwrap(),
            };

            let mut app = App::new(renderer, synchronizer, clock_offset, player).await;
            app.update_track_and_subtitle_document_information().await;

            let mpris = app.mpris.clone();
            let mut events = mpris.events().await?;
            let mut tick = tokio::time::interval(std_duration::from_secs_f64(1f64 / fps));
            let mut keyboard = EventStream::new();

            loop {
                tokio::select! {
                    Some(event) = events.next() => app.handle_player_event(event).await?,

                    _ = tick.tick() => app.handle_tick_event().await?,

                    Some(Ok(Event::Key(key))) = keyboard.next() => keyboard::handle_keyboard_event(&mut app, key, &config).await?,
                }

                if app.state.quit == true {
                    break Ok(());
                }
            }
        }
    }
}
