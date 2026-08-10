use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer};
use subtitles::subtitles::SubtitleContent;
use synchronizer::traits::Synchronizer;

pub fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    cue_index: usize,
    _config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match &mut app.state.subtitle_document {
        Some(document) => {
            let current_cue = &mut document.cues[cue_index];
            match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => app.state.quit = true,

                KeyCode::Esc => {
                    // undo all changes as well
                    app.switch_to_normal_mode()
                },

                KeyCode::Tab => app.switch_to_normal_mode(),
                KeyCode::Enter => app.switch_to_select_mode()?,

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
        },
        None => app.switch_to_normal_mode(),
    }

    Ok(())
}

