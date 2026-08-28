use chrono::Duration;
use synchronizer::traits::CueIndexed;

use crate::state::AppState;

pub trait Renderer<A: CueIndexed> {
    type Error: std::error::Error + 'static;

    fn render(
        &mut self,
        state: &mut AppState,
        position: Option<Duration>,
        active_cues: &[A],
    ) -> Result<(), Self::Error>;

    fn get_lines_per_page(&self) -> usize;
}
