use chrono::Duration;

use crate::state::AppState;

pub trait Renderer {
    type Error;

    fn render(&mut self, state: &AppState, position: Duration) -> Result<(), Self::Error>;
}
