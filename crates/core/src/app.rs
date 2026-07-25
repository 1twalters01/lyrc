use mpris::{client::MprisClient, playback::PlayerEvent};
use synchronizer::traits::Synchronizer;

use crate::{clock::PlaybackClock, renderer::Renderer, state::AppState};

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
    pub fn handle_player_event(&mut self, event: PlayerEvent) -> Result<(), <R as Renderer>::Error>{
        self.state.update(&event);
        self.clock.update(event);

        let subtitle_document = &self.state.subtitle_document;
        let position = &self.clock.get_position();
        if let Some(subtitles) = subtitle_document {
            self.synchronizer.update(subtitles, position);
        }


        self.renderer.render(&self.state)
    }

    pub async fn handle_tick(&mut self) -> Result<(), String> {
        let current_position = self.mpris.get_current_position().await.unwrap();
        let playback_status = self.mpris.get_playback_status().await.unwrap();
        self.clock.sync(current_position, playback_status).await
    }
}

