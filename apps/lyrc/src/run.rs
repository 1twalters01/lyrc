use crate::args::{Args, Command, Frontend};
use chrono::Duration;
use futures_util::stream::StreamExt;
use lyrc_core::app::App;
use lyrics::{
    models::{Lyrics, LyricsFormat},
    service::LyricsService,
};
use std::time::Duration as std_duration;
use subtitles::{
    formats::lrc::parser::LrcParser, parser::SubtitleParser, subtitles::SubtitleDocument,
};
use synchronizer::strategies::lyrics::LyricsSynchronizer;
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    // println!("Initialisation code");
    let hz = 60;
    let player = "cmus"; // also want to be able to automatically find a player
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
                    Some(event) = events.next() => app.handle_player_event(event).await?,

                    _ = tick.tick() => app.tick().await?,

                    Some(Ok(Event::Key(key))) = keyboard.next() => {
                        match key.code {
                            // Quit
                            KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => break Ok(()),

                            // playback control
                            KeyCode::Char(' ') => app.toggle_play_pause().await?,
                            KeyCode::Left => app.seek_by_duration(rewind_duration).await?,
                            KeyCode::Right => app.seek_by_duration(fast_forward_duration).await?,

                            // line control
                            KeyCode::Char('k') => app.select_previous_line(),
                            KeyCode::Char('j') => app.select_next_line(),
                            KeyCode::Char('h') => app.clear_line_selection(),
                            KeyCode::Enter => app.seek_to_selected_line().await?,

                            // download lyrics
                            KeyCode::Char('d') => {
                                if app.state.subtitle_document.is_none() {
                                    // store in app? and have app.lyrics_service or something?
                                    let lyrics_service = LyricsService::default();
                                    let lyrics_provider = lyrics_service.providers.get("lrclib");
                                    let track = app.state.track.clone();
                                    let subtitle_document = match (lyrics_provider, track) {
                                        (Some(provider), Some(track)) => {
                                            let lyrics = provider.search(track.clone()).await?;
                                            if let Some(lyrics) = lyrics {
                                                match lyrics.format {
                                                    LyricsFormat::Lrc => {
                                                        if let Some(file_path) = track.file_path {
                                                            let mut lrc_path = file_path.to_path_buf();
                                                            lrc_path.set_extension("lrc");
                                                            println!("lrc path: {:?}", lrc_path);

                                                            std::fs::write(&lrc_path, &lyrics.content)?;
                                                        }
                                                        Some(LrcParser.parse(&lyrics.content)?)
                                                    },
                                                    LyricsFormat::Text => None,
                                                }
                                            } else { None }
                                        },
                                        (_, _) => None,
                                    };

                                    app.state.subtitle_document = subtitle_document;
                                }
                            }

                            _ => {},
                        }
                    },
                }
            }
        }
    }
}
