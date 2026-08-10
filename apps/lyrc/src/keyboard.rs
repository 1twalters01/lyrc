use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer};
use lyrics::{models::LyricsFormat, service::LyricsService};
use subtitles::{
    formats::lrc::parser::LrcParser, parser::SubtitleParser, subtitles::SubtitleContent,
};
use synchronizer::traits::Synchronizer;

pub async fn handle_normal_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        // Quit
        KeyCode::Esc => if app.state.selected_cue.is_some() { app.clear_line_selection(); } else { app.state.quit = true },
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => app.state.quit = true,
        KeyCode::Char('q') => app.state.quit = true,

        // playback control
        KeyCode::Char(' ') => app.toggle_play_pause().await?,
        KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
        KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

        // line control
        KeyCode::Char('k') => app.select_previous_line(),
        KeyCode::Char('j') => app.select_next_line(),
        KeyCode::Char('u') => app.clear_line_selection(),
        KeyCode::Enter => app.seek_to_selected_line().await?,

        // Edit subtitle cue
        KeyCode::Tab => app.switch_to_edit_mode(),

        KeyCode::Char('m') => {
            app.adjust_all_cues_start_backwards(config.backwards_cue_increment_small)
        }
        KeyCode::Char('/') => {
            app.adjust_all_cues_start_forwards(config.forwards_cue_increment_small)
        }
        KeyCode::Char('M') => {
            app.adjust_all_cues_start_backwards(config.backwards_cue_increment_large)
        }
        KeyCode::Char('?') => {
            app.adjust_all_cues_start_forwards(config.forwards_cue_increment_large)
        }

        KeyCode::Char(',') => {
            app.adjust_selected_cue_start_backwards(config.backwards_cue_increment_small)
        }
        KeyCode::Char('.') => {
            app.adjust_selected_cue_start_forwards(config.forwards_cue_increment_small)
        }
        KeyCode::Char('<') => {
            app.adjust_selected_cue_start_backwards(config.backwards_cue_increment_large)
        }
        KeyCode::Char('>') => {
            app.adjust_selected_cue_start_forwards(config.forwards_cue_increment_large)
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

pub fn handle_edit_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    _config: &crate::config::Config,
) {
    match (
        &mut app.state.subtitle_document,
        &mut app.state.selected_cue,
    ) {
        (Some(document), Some(line_idx)) => {
            let current_cue = &mut document.cues[*line_idx];
            match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                    app.state.quit = true
                }
                KeyCode::Tab | KeyCode::Esc => app.switch_to_normal_mode(),
                KeyCode::Char(char) => match &mut current_cue.content {
                    SubtitleContent::Text(text) => text.push(char),
                },
                KeyCode::Backspace => match &mut current_cue.content {
                    SubtitleContent::Text(text) => {
                        text.pop();
                    }
                },
                _ => {}
            }
        }
        _ => app.switch_to_normal_mode(),
    }
}


pub fn handle_select_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    _config: &crate::config::Config,
) {
}
