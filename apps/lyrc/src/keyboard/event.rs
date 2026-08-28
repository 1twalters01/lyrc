use crossterm::event::KeyEvent;
use lyrc_core::{app::App, mode::AppMode, renderer::Renderer};
use synchronizer::traits::Synchronizer;

use crate::{config::Config, keyboard};

pub async fn handle_keyboard_event<R: Renderer<S::Active>, S: Synchronizer>(
    app: &mut App<R, S>,
    key: KeyEvent,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mode = &match app.state.app_mode {
        AppMode::Normal => AppMode::Normal,
        AppMode::Select {
            cue_index,
            ref selected_cues,
        } => AppMode::Select {
            cue_index,
            selected_cues: selected_cues.clone(),
        },
        AppMode::Edit {
            cue_index,
            ref selected_cues,
        } => AppMode::Edit {
            cue_index,
            selected_cues: selected_cues.clone(),
        },
    };

    match mode {
        AppMode::Normal => keyboard::normal::handle_key(app, key, &config).await,
        AppMode::Select {
            cue_index,
            selected_cues: _,
        } => keyboard::select::handle_key(app, key, *cue_index, &config).await,
        AppMode::Edit {
            cue_index,
            selected_cues,
        } => keyboard::edit::handle_key(app, key, *cue_index, selected_cues.clone(), &config),
    }
}
