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
                        document.save()?;
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
            KeyCode::Enter => app.seek_to_selected_line(cue_index).await?,

            // Playback control
            KeyCode::Char(' ') => app.toggle_play_pause().await?,
            KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
            KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

            // Line control
            KeyCode::Up => app.go_to_previous_line(),
            KeyCode::Char('k') => app.go_to_previous_line(),
            KeyCode::Down => app.go_to_next_line(),
            KeyCode::Char('j') => app.go_to_next_line(),
            KeyCode::Char('H') => app.toggle_select_all_lines()?,
            KeyCode::Char('h') => app.toggle_select_line(),
            KeyCode::Char('D') => app.delete_selected_lines(),
            KeyCode::Char('d') => app.delete_current_line(),

            // Adjust cue time
            KeyCode::Char(',') => {
                app.state.unsaved_changes = true;
                app.decrease_selected_cue_start_time(config.backwards_cue_increment_small)?
            }
            KeyCode::Char('.') => {
                app.state.unsaved_changes = true;
                app.increase_selected_cue_start_time(config.forwards_cue_increment_small)?
            }
            KeyCode::Char('<') => {
                app.state.unsaved_changes = true;
                app.decrease_selected_cue_start_time(config.backwards_cue_increment_large)?
            }
            KeyCode::Char('>') => {
                app.state.unsaved_changes = true;
                app.increase_selected_cue_start_time(config.forwards_cue_increment_large)?
            }

            _ => {}
        },
        None => app.switch_to_normal_mode(),
    }

    Ok(())
}
