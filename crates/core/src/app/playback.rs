use chrono::Duration;
use mpris::playback::{PlaybackCommand, PlaybackStatus};
use synchronizer::traits::Synchronizer;

use crate::{app::App, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub async fn get_current_position(&self) -> Option<Duration> {
        self.mpris.get_current_position().await.ok()
    }

    pub async fn get_playback_status(&self) -> PlaybackStatus {
        self.mpris
            .get_playback_status()
            .await
            .unwrap_or(PlaybackStatus::Unknown)
    }

    pub async fn toggle_play_pause(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.mpris.execute(PlaybackCommand::Toggle).await?;
        Ok(())
    }

    pub async fn seek_by_duration(
        &mut self,
        duration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.mpris.execute(PlaybackCommand::Seek(duration)).await?;
        Ok(())
    }

    // Use a better error type
    pub async fn seek_to_selected_line(
        &mut self,
        cue_index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.track.is_none() {
            return Err(String::from("No track was found").into());
        }

        if let Some(ref document) = self.state.subtitle_document {
            let cue = &document.cues[cue_index];
            let duration = cue.start;
            self.mpris
                .execute(PlaybackCommand::SetPosition(duration))
                .await?;
        } else {
            return Err(String::from("No subtitle_document was found").into());
        }

        Ok(())
    }
}
