use std::io;

use chrono::Duration;
use lyrc_core::renderer::Renderer;

pub struct GuiRenderer {
    lines_per_page: usize,
}

impl GuiRenderer {
    pub fn new() -> io::Result<Self> {
        let lines_per_page = 0;

        Ok(Self { lines_per_page })
    }
}

impl Renderer for GuiRenderer {
    type Error = io::Error;

    fn render(
        &mut self,
        state: &mut lyrc_core::state::AppState,
        position: Option<Duration>,
        active_indices: &Vec<lyrc_core::synchronizer::ActiveIndex>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_lines_per_page(&self) -> usize {
        self.lines_per_page
    }
}
