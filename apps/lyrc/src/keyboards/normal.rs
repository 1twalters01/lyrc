use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer, mode::AppMode};
use lyrics::{models::LyricsFormat, service::LyricsService};
use subtitles::{
    formats::lrc::parser::LrcParser, parser::SubtitleParser, subtitles::SubtitleDocument,
};
use synchronizer::traits::Synchronizer;

pub async fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        // Quit
        KeyCode::Esc => {
            if app.state.unsaved_changes {
                app.state.unsaved_changes = false;
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
            } else {
                app.state.quit = true
            }
        }
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => app.state.quit = true,

        // Save
        KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
            match &app.state.subtitle_document {
                Some(document) => {
                    app.save_document(document.clone())?;
                    app.state.unsaved_changes = false;
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
                }
                None => {}
            }
        }

        // playback control
        KeyCode::Char(' ') => app.toggle_play_pause().await?,
        KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
        KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

        // line control
        KeyCode::Up => match app.get_first_active_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },
        KeyCode::Down => match app.get_first_active_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },
        KeyCode::Char('k') => match app.get_first_active_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },
        KeyCode::Char('j') => match app.get_first_active_cue() {
            Some(cue_index) => app.state.app_mode = AppMode::Select { cue_index },
            None => {}
        },

        // Change modes
        KeyCode::Enter => app.switch_to_select_mode()?,
        KeyCode::Tab => app.switch_to_select_mode()?,

        // Bulk adjust cue times
        KeyCode::Char(',') => {
            app.state.unsaved_changes = true;
            app.adjust_all_cues_start_backwards(config.backwards_cue_increment_small)
        }
        KeyCode::Char('.') => {
            app.state.unsaved_changes = true;
            app.adjust_all_cues_start_forwards(config.forwards_cue_increment_small)
        }
        KeyCode::Char('<') => {
            app.state.unsaved_changes = true;
            app.adjust_all_cues_start_backwards(config.backwards_cue_increment_large)
        }
        KeyCode::Char('>') => {
            app.state.unsaved_changes = true;
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

        // _ => eprintln!("{:?}", key),
        _ => {}
    }

    Ok(())
}
