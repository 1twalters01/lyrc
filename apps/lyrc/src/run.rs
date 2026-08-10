use std::time::Duration as std_duration;

use crate::{
    args::{Args, Command, Frontend},
    keyboards,
};

use lyrc_core::{app::App, state::AppMode};
use subtitles::subtitles::SubtitleDocument;
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
    let fps = 60;
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
                        let mode = &match app.state.app_mode {
                            AppMode::Normal => AppMode::Normal,
                            AppMode::Select { cue_index } => AppMode::Select { cue_index },
                            AppMode::Edit { cue_index, ref original_content } => AppMode::Edit { cue_index, original_content: original_content.clone() },
                        };

                        match mode {
                            AppMode::Normal => keyboards::normal::handle_key(&mut app, key, &config).await?,
                            AppMode::Select { cue_index } => keyboards::select::handle_key(&mut app, key, *cue_index, &config).await?,
                            AppMode::Edit { cue_index, original_content } => keyboards::edit::handle_key(&mut app, key, *cue_index, original_content.clone(), &config)?,
                        }
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
