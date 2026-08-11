use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer, state::AppMode};
use subtitles::subtitles::{SubtitleContent, SubtitleDocument};
use synchronizer::traits::Synchronizer;

pub fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    cue_index: usize,
    original_content: SubtitleContent,
    _config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match &mut app.state.subtitle_document {
        Some(document) => {
            let current_cue = &mut document.cues[cue_index];
            match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                    app.state.quit = true
                }

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
                                        let subtitle_document =
                                            SubtitleDocument::from_pathbuf(lyrics_path)?;

                                        let original_content =
                                            subtitle_document.cues[cue_index].content.clone();
                                        app.state.app_mode = AppMode::Edit {
                                            cue_index,
                                            original_content,
                                        };
                                        Some(subtitle_document)
                                    }
                                    None => None,
                                },
                                None => None,
                            };
                        }
                        None => {}
                    }
                }

                KeyCode::Esc => {
                    document.cues[cue_index].content = original_content;
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
                    app.switch_to_select_mode()?
                }

                KeyCode::Tab => app.switch_to_normal_mode(),
                KeyCode::Enter => app.switch_to_select_mode()?,

                KeyCode::Char(char) => match &mut current_cue.content {
                    SubtitleContent::Text(text) => {
                        app.state.unsaved_changes = true;
                        text.push(char);
                    }
                },
                KeyCode::Backspace => match &mut current_cue.content {
                    SubtitleContent::Text(text) => {
                        app.state.unsaved_changes = true;
                        text.pop();
                    }
                },

                _ => {}
            }
        }
        None => app.switch_to_normal_mode(),
    }

    Ok(())
}
