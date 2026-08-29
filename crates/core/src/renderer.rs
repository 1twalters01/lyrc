use chrono::Duration;

use crate::{state::AppState, synchronizer::ActiveIndex};

pub trait Renderer {
    type Error: std::error::Error + 'static;

    fn render(
        &mut self,
        state: &mut AppState,
        position: Option<Duration>,
        active_indices: &Vec<ActiveIndex>,
    ) -> Result<(), Self::Error>;

    fn get_lines_per_page(&self) -> usize;
}
