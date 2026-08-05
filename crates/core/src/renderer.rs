use chrono::Duration;

use crate::state::AppState;

pub trait Renderer {
    type Error;

    fn render(
        &mut self,
        state: &AppState,
        position: Option<Duration>,
        active_cues: &[usize],
    ) -> Result<(), Self::Error>;
}
