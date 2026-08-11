use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer};
use subtitles::subtitles::SubtitleDocument;
use synchronizer::traits::Synchronizer;

pub async fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    cue_index: usize,
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match &mut app.state.subtitle_document {
        Some(_document) => match key.code {
            // Quit
            KeyCode::Char('q') => app.state.quit = true,
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                app.state.quit = true;
            }

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

            // Mode change
            KeyCode::Esc => {
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
                app.switch_to_normal_mode()
            }
            KeyCode::Tab => app.switch_to_edit_mode()?,
            KeyCode::Enter => {
                if Some(cue_index) == app.get_first_active_cue() {
                    app.switch_to_edit_mode()?
                } else {
                    app.seek_to_selected_line(cue_index).await?
                }
            }

            // Playback control
            KeyCode::Char(' ') => app.toggle_play_pause().await?,
            KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
            KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

            // Line control
            KeyCode::Up => app.select_previous_line(cue_index),
            KeyCode::Char('k') => app.select_previous_line(cue_index),
            KeyCode::Down => app.select_next_line(cue_index),
            KeyCode::Char('j') => app.select_next_line(cue_index),

            // Adjust cue time
            KeyCode::Char(',') => {
                app.state.unsaved_changes = true;
                app.adjust_selected_cue_start_backwards(config.backwards_cue_increment_small)?
            }
            KeyCode::Char('.') => {
                app.state.unsaved_changes = true;
                app.adjust_selected_cue_start_forwards(config.forwards_cue_increment_small)?
            }
            KeyCode::Char('<') => {
                app.state.unsaved_changes = true;
                app.adjust_selected_cue_start_backwards(config.backwards_cue_increment_large)?
            }
            KeyCode::Char('>') => {
                app.state.unsaved_changes = true;
                app.adjust_selected_cue_start_forwards(config.forwards_cue_increment_large)?
            }

            _ => {}
        },
        None => app.switch_to_normal_mode(),
    }

    Ok(())
}
