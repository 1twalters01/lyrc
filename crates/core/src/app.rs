use mpris::client::MprisClient;
use synchronizer::traits::Synchronizer;

use crate::{renderer::Renderer, state::AppState};

pub struct App<R, S>
where 
        R: Renderer,
        S: Synchronizer,
{
    renderer: R,
    state: AppState,
    synchronizer: S,
    mpris: MprisClient,
}
