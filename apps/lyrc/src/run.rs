use crate::{
    args::{Args, Command, Frontend},
    keyboard,
};
use chrono::Duration;
use futures_util::stream::StreamExt;
use lyrc_core::{app::App, state::AppMode};
use lyrics::{models::LyricsFormat, service::LyricsService};
use std::time::Duration as std_duration;
use subtitles::{
    formats::lrc::parser::LrcParser,
    parser::SubtitleParser,
    subtitles::{SubtitleContent, SubtitleDocument},
};
use synchronizer::{strategies::lyrics::LyricsSynchronizer, traits::Synchronizer};
use tui::renderer::TuiRenderer;

use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    // println!("Initialisation code");
    let fps = 60;
    let player = "cmus"; // also want to be able to automatically find a player
    let clock_offset = Duration::milliseconds(0);
    let rewind_duration = Duration::milliseconds(-5000);
    let fast_forward_duration = Duration::milliseconds(5000);
    let forwards_cue_increment = Duration::milliseconds(10);
    let backwards_cue_increment = Duration::milliseconds(10);

    let config = crate::config::Config {
        rewind_duration,
        fast_forward_duration,
        forwards_cue_increment,
        backwards_cue_increment,
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

            let mut tick = tokio::time::interval(std_duration::from_secs_f64(1.0 / fps as f64));

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
                        // if app.state.is_editing_cue {
                        //     if key.code == KeyCode::Tab || key.code == KeyCode::Esc {
                        //         app.state.is_editing_cue = false;
                        //     } else {
                        //         match (&mut app.state.subtitle_document, &mut app.state.selected_cue) {
                        //             (Some(document), Some(line_idx)) => {
                        //                 let current_cue = &mut document.cues[*line_idx];
                        //                 match key.code {
                        //                     KeyCode::Char(c) => {
                        //                         match &mut current_cue.content{
                        //                             SubtitleContent::Text(text) => text.push(c),
                        //                         }
                        //                     }
                        //                     KeyCode::Backspace => match &mut current_cue.content {
                        //                         SubtitleContent::Text(text) => {
                        //                             text.pop();
                        //                         },
                        //                     },
                        //                     _ => {},
                        //                 }
                        //             },
                        //             _ => {},
                        //         }
                        //     }
                        // } else {
                        match app.state.app_mode {
                            AppMode::Normal => crate::keyboard::handle_normal_key(&mut app, key, &config).await?,
                            AppMode::Edit => crate::keyboard::handle_edit_key(&mut app, key, &config),
                        }
                        // }
                    },
                }

                if app.state.quit == true {
                    break;
                }
            }

            Ok(())
        }
    }
}
