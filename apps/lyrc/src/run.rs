use crate::args::{Args, Command, Frontend};
use chrono::Duration;
use futures_util::stream::StreamExt;
use lyrc_core::app::App;
use mpris::playback::PlaybackCommand;
use std::time::Duration as std_duration;
use synchronizer::strategies::lyrics::LyricsSynchronizer;
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream, KeyCode};

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    // println!("Initialisation code");
    let hz = 60;
    let clock_offset = Duration::milliseconds(0);
    let rewind_duration = Duration::milliseconds(-5000);
    let fast_forward_duration = Duration::milliseconds(5000);

    match command {
        Command::Daemon => {
            println!("daemon");
            Ok(())
        }
        Command::App { frontend } => {
            let synchronizer = LyricsSynchronizer::new();
            let player = "cmus";

            let renderer = match frontend {
                Frontend::Tui => TuiRenderer::new().unwrap(),
            };

            let mut app = App::new(renderer, synchronizer, clock_offset, player).await;

            let mut tick = tokio::time::interval(std_duration::from_secs_f64(1.0 / hz as f64));

            let mpris = app.mpris.clone();

            app.state.track = Some(mpris.get_current_track().await?);

            let mut keyboard = EventStream::new();
            let mut events = mpris.events().await?;

            loop {
                tokio::select! {
                    Some(event) = events.next() => {
                        app.handle_player_event(event)?
                    }

                    _ = tick.tick() => {
                        let current_position = app.get_current_position().await;
                        let playback_status = app.get_playback_status().await;
                        app.handle_tick(current_position, playback_status)?
                    }

                    Some(Ok(Event::Key(key))) = keyboard.next() => {
                        if key.code == KeyCode::Char('q') {
                            break Ok(());
                        } else if key.code == KeyCode::Left {
                            mpris.execute(PlaybackCommand::Seek(rewind_duration)).await?
                        } else if key.code == KeyCode::Right {
                            mpris.execute(PlaybackCommand::Seek(fast_forward_duration)).await?
                        }
                    }
                }
            }
        }
    }
}
