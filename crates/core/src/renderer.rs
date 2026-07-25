use crate::state::AppState;

pub trait Renderer {
    type Target;
    type Error;

    // fn render(&mut self, target: Self::Target, state: &AppState) -> Result<(), Self::Error>;
    fn render(&mut self, state: &AppState) -> Result<(), Self::Error>;
}
