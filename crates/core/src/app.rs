use mpris::{client::MprisClient, playback::PlayerEvent};
use synchronizer::traits::Synchronizer;

use crate::{clock::PlaybackClock, events::AppEvent, renderer::Renderer, state::AppState};

pub struct App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    renderer: R,
    clock: PlaybackClock,
    state: AppState,
    synchronizer: S,
    mpris: MprisClient,
}

impl<R, S> App<R, S>
where 
    R: Renderer,
    S: Synchronizer,
{
    pub fn handle_event(&mut self, event: PlayerEvent) -> Result<(), <R as Renderer>::Error>{
        self.state.update(&event);

        let subtitles = &self.state.subtitles;
        let position = &self.state.playback_state.position;
        if let Some(subtitles) = subtitles {
            self.synchronizer.update(subtitles, position);
        }

        self.clock.update(&event);

        self.renderer.render(&self.state)
    }

    pub fn handle_tick(&mut self) {
    }
}


