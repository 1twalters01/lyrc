use crate::state::AppState;

pub trait Renderer {
    type Error;

    fn render(&mut self, state: &AppState) -> Result<(), Self::Error>;
}
