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
        Some(document) => match key.code {
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

            // Undo and redo changes
            KeyCode::Char('z') if key.modifiers == KeyModifiers::CONTROL => app.undo(),
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => app.redo(),

            // Mode change
            KeyCode::Esc => {
                if app.state.unsaved_changes == true {
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
                    app.switch_to_normal_mode()
                }
            }
            KeyCode::Tab => app.switch_to_edit_mode()?,
            KeyCode::Enter => app.seek_to_selected_line(cue_index).await?,

            // Playback control
            KeyCode::Char(' ') => app.toggle_play_pause().await?,
            KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
            KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

            // Line control
            KeyCode::Up if key.modifiers == KeyModifiers::CONTROL => app.go_to_previous_half_page(),
            KeyCode::Down if key.modifiers == KeyModifiers::CONTROL => app.go_to_next_half_page(),
            KeyCode::Up => app.go_to_previous_line(),
            KeyCode::Down => app.go_to_next_line(),
            KeyCode::Char('H') => app.toggle_select_all_lines()?,
            KeyCode::Char('h') => app.toggle_select_line(),
            KeyCode::Char('D') => app.delete_selected_lines(),
            KeyCode::Char('d') => app.delete_current_line(),
            KeyCode::Char('k') => app.add_cue_before_current_cue(),
            KeyCode::Char('j') => app.add_cue_after_current_cue(),
            KeyCode::Char('o') => app.add_cue_before_selected_cues(),
            KeyCode::Char('i') => app.add_cue_after_selected_cues(),

            // Adjust cue time
            KeyCode::Char('m') => {
                app.state.unsaved_changes = true;
                app.decrease_current_cue_start_time(config.backwards_cue_increment_small)?
            }
            KeyCode::Char(',') => {
                app.state.unsaved_changes = true;
                app.increase_current_cue_start_time(config.forwards_cue_increment_small)?
            }
            KeyCode::Char('.') => {
                app.state.unsaved_changes = true;
                app.decrease_current_cue_end_time(config.backwards_cue_increment_small)?
            }
            KeyCode::Char('/') => {
                app.state.unsaved_changes = true;
                app.increase_current_cue_end_time(config.forwards_cue_increment_small)?
            }
            KeyCode::Char('M') => {
                app.state.unsaved_changes = true;
                app.decrease_current_cue_start_time(config.backwards_cue_increment_large)?
            }
            KeyCode::Char('<') => {
                app.state.unsaved_changes = true;
                app.increase_current_cue_start_time(config.forwards_cue_increment_large)?
            }
            KeyCode::Char('>') => {
                app.state.unsaved_changes = true;
                app.decrease_current_cue_end_time(config.backwards_cue_increment_large)?
            }
            KeyCode::Char('?') => {
                app.state.unsaved_changes = true;
                app.increase_current_cue_end_time(config.forwards_cue_increment_large)?
            }
            KeyCode::Char('c') => match app.clock.get_position() {
                Some(position) => {
                    app.state.unsaved_changes = true;
                    app.set_current_cue_start_time(position)?
                }
                None => {}
            },
            KeyCode::Char('C') => match app.clock.get_position() {
                Some(position) => {
                    app.state.unsaved_changes = true;
                    app.set_current_cue_end_time(position)?
                }
                None => {}
            },

            _ => {}
        },
        None => app.switch_to_normal_mode(),
    }

    Ok(())
}
