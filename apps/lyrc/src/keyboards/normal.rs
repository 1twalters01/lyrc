use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer, state::AppMode};
use lyrics::{models::LyricsFormat, service::LyricsService};
use subtitles::{formats::lrc::parser::LrcParser, parser::SubtitleParser};
use synchronizer::traits::Synchronizer;

pub async fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        // Quit
        KeyCode::Esc => app.state.quit = true,
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => app.state.quit = true,
        KeyCode::Char('q') => app.state.quit = true,

        // playback control
        KeyCode::Char(' ') => app.toggle_play_pause().await?,
        KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
        KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

        // line control
        KeyCode::Up => match app.get_first_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },
        KeyCode::Char('k') => match app.get_first_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },
        KeyCode::Down => match app.get_first_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },
        KeyCode::Char('j') => match app.get_first_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },

        // Change modes
        KeyCode::Tab => app.switch_to_select_mode()?,

        KeyCode::Char(',') => {
            app.adjust_all_cues_start_backwards(config.backwards_cue_increment_small)
        }
        KeyCode::Char('.') => {
            app.adjust_all_cues_start_forwards(config.forwards_cue_increment_small)
        }
        KeyCode::Char('<') => {
            app.adjust_all_cues_start_backwards(config.backwards_cue_increment_large)
        }
        KeyCode::Char('>') => {
            app.adjust_all_cues_start_forwards(config.forwards_cue_increment_large)
        }

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
                                }
                                LyricsFormat::Text => None,
                            }
                        } else {
                            None
                        }
                    }
                    (_, _) => None,
                };

                app.state.subtitle_document = subtitle_document;
            }
        }

        _ => eprintln!("{:?}", key),
        // _ => panic!("{:?}", key),
        // _ => {}
    }

    Ok(())
}
