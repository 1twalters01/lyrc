use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{
    app::App,
    mode::{AppMode, EditCue},
    renderer::Renderer,
};
use subtitles::subtitles::{SubtitleContent, SubtitleDocument};
use synchronizer::traits::Synchronizer;

pub fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    cue_index: usize,
    selected_cues: Vec<EditCue>,
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
                            document.save()?;
                            app.state.unsaved_changes = false;
                            app.state.subtitle_document = match app.state.track {
                                Some(ref track) => match &track.file_path {
                                    Some(file_path) => {
                                        let mut lyrics_path = file_path.to_path_buf();
                                        lyrics_path.set_extension("lrc");
                                        let subtitle_document =
                                            SubtitleDocument::from_pathbuf(lyrics_path)?;

                                        let selected_cues = selected_cues
                                            .iter()
                                            .map(|c| EditCue {
                                                index: c.index,
                                                original_content: subtitle_document.cues[c.index]
                                                    .content
                                                    .clone(),
                                            })
                                            .collect();

                                        app.state.app_mode = AppMode::Edit {
                                            cue_index,
                                            selected_cues,
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
                    if app.state.unsaved_changes == true {
                        for cue in selected_cues {
                            document.cues[cue.index].content = cue.original_content;
                        }

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
                        app.switch_to_select_mode()?
                    }
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
