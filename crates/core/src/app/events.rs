use chrono::Duration;
use mpris::playback::{PlaybackStatus, PlayerEvent};
use synchronizer::traits::Synchronizer;

use crate::{app::App, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub async fn handle_player_event(
        &mut self,
        event: PlayerEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.state.update(&event);
        self.clock.update(event);

        let subtitle_document = &self.state.subtitle_document;
        let position = self.clock.get_position();
        self.synchronizer.update(subtitle_document, &position);

        self.render()?;
        Ok(())
    }

    pub async fn handle_tick_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let current_position = self.get_current_position().await;
        let playback_status = self.get_playback_status().await;

        self.process_tick(current_position, playback_status)
    }

    fn process_tick(
        &mut self,
        current_position: Option<Duration>,
        playback_status: PlaybackStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clock.sync(current_position, playback_status)?;

        let subtitle_document = &self.state.subtitle_document;
        let position = self.clock.get_position();
        self.synchronizer.update(subtitle_document, &position);

        self.render()?;
        Ok(())
    }

    fn render(&mut self) -> Result<(), R::Error> {
        let position = self.clock.get_position();
        let active_cues = self.synchronizer.get_active_cues();
        self.renderer.render(&mut self.state, position, active_cues)
    }
}
