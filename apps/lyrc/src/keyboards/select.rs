use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lyrc_core::{app::App, renderer::Renderer};
use synchronizer::traits::Synchronizer;

pub async fn handle_key<R: Renderer, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    cue_index: usize,
    config: &crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match &mut app.state.subtitle_document {
        Some(_document) => match key.code {
            KeyCode::Char('q') => app.state.quit = true,
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                app.state.quit = true;
            }

            KeyCode::Esc => app.switch_to_normal_mode(),
            KeyCode::Tab => app.switch_to_edit_mode()?,
            KeyCode::Enter => app.seek_to_selected_line(cue_index).await?,

            // playback control
            KeyCode::Char(' ') => app.toggle_play_pause().await?,
            KeyCode::Left => app.seek_by_duration(config.rewind_duration).await?,
            KeyCode::Right => app.seek_by_duration(config.fast_forward_duration).await?,

            KeyCode::Up => app.select_previous_line(cue_index),
            KeyCode::Char('k') => app.select_previous_line(cue_index),
            KeyCode::Down => app.select_next_line(cue_index),
            KeyCode::Char('j') => app.select_next_line(cue_index),

            KeyCode::Char(',') => {
                app.adjust_selected_cue_start_backwards(config.backwards_cue_increment_small)?
            }
            KeyCode::Char('.') => {
                app.adjust_selected_cue_start_forwards(config.forwards_cue_increment_small)?
            }
            KeyCode::Char('<') => {
                app.adjust_selected_cue_start_backwards(config.backwards_cue_increment_large)?
            }
            KeyCode::Char('>') => {
                app.adjust_selected_cue_start_forwards(config.forwards_cue_increment_large)?
            }

            _ => {}
        },
        None => app.switch_to_normal_mode(),
    }

    Ok(())
}
