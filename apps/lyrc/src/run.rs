use crate::args::{Args, Command, Frontend};
use chrono::Duration;
use futures_util::stream::StreamExt;
use lyrc_core::app::App;
use mpris::playback::PlaybackCommand;
use std::time::Duration as std_duration;
use subtitles::subtitles::SubtitleDocument;
use synchronizer::strategies::lyrics::LyricsSynchronizer;
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    // println!("Initialisation code");
    let hz = 60;
    let player = "cmus";
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

            let renderer = match frontend {
                Frontend::Tui => TuiRenderer::new().unwrap(),
            };

            let mut app = App::new(renderer, synchronizer, clock_offset, player).await;

            let mut tick = tokio::time::interval(std_duration::from_secs_f64(1.0 / hz as f64));

            let mpris = app.mpris.clone();

            app.state.track = mpris.get_current_track().await.ok();
            app.state.subtitle_document = match app.state.track {
                Some(ref track) => match &track.file_path {
                    Some(file_path) => {
                        let mut lyrics_path = file_path.to_path_buf();
                        lyrics_path.set_extension("lrc");
                        SubtitleDocument::from_pathbuf(lyrics_path).ok()
                    }
                    None => None,
                },
                None => None,
            };

            let mut keyboard = EventStream::new();
            let mut events = mpris.events().await?;

            loop {
                tokio::select! {
                    Some(event) = events.next() => {
                        app.handle_player_event(event).await?
                    }

                    _ = tick.tick() => {
                        let current_position = app.get_current_position().await;
                        let playback_status = app.get_playback_status().await;
                        app.handle_tick(current_position, playback_status)?
                    }

                    Some(Ok(Event::Key(key))) = keyboard.next() => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => break Ok(()),

                            KeyCode::Char(' ') => mpris
                                .execute(PlaybackCommand::Toggle)
                                .await?,
                            KeyCode::Left => mpris
                                .execute(PlaybackCommand::Seek(rewind_duration))
                            .await?,
                            KeyCode::Right => mpris
                                .execute(PlaybackCommand::Seek(fast_forward_duration))
                            .await?,

                            KeyCode::Char('k') => {
                                let offset = app.state.manual_scroll_offset
                                    .get_or_insert(app.state.automatic_scroll_offset);
                                *offset = offset.saturating_sub(1);
                            }
                            KeyCode::Char('j') => {
                                let offset = app.state.manual_scroll_offset
                                    .get_or_insert(app.state.automatic_scroll_offset);
                                *offset += 1;
                            }

                            KeyCode::Up => {
                                let selected_line = app.state.selected_line
                                    .get_or_insert(
                                        *app.state.manual_scroll_offset
                                            .get_or_insert(app.state.automatic_scroll_offset)
                                    );
                                *selected_line = selected_line.saturating_sub(1);
                            },
                            KeyCode::Down => {
                                let selected_line = app.state.selected_line
                                    .get_or_insert(
                                        *app.state.manual_scroll_offset
                                            .get_or_insert(app.state.automatic_scroll_offset)
                                    );
                                *selected_line += 1;
                            },

                            KeyCode::Char('h') => {
                                app.state.manual_scroll_offset = None;
                                app.state.selected_line = None;
                            },

                            _ => {},
                        }
                    }
                }
            }
        }
    }
}
