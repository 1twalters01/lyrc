use chrono::Duration;
use lyrc_core::{renderer::Renderer, state::AppState};
use std::io::{self, Stdout};

use ratatui::{
    Terminal,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

use crate::draw::window::draw_window;

pub struct TuiRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiRenderer {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;

        let mut stdout = std::io::stdout();

        execute!(stdout, EnterAlternateScreen,)?;

        let backend = CrosstermBackend::new(std::io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }
}

impl Renderer for TuiRenderer {
    type Error = std::io::Error;

    fn render(
        &mut self,
        state: &AppState,
        position: Option<Duration>,
        active_cues: &[usize],
    ) -> Result<(), Self::Error> {
        self.terminal.draw(|frame| {
            draw_window(frame, state, position, active_cues);
        })?;

        Ok(())
    }
}

impl Drop for TuiRenderer {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen,);
    }
}
