use std::time::Instant;

use chrono::Duration;
use mpris::playback::{PlaybackStatus, PlayerEvent};

pub struct PlaybackClock {
    last_position: Duration,
    interpolated_duration: Duration,
    last_sync: Instant,
    previous_playback_status: Option<PlaybackStatus>,
}

impl PlaybackClock {
    pub fn get_position(&self) -> Duration {
        self.last_position + self.interpolated_duration
    }

    pub fn update(&mut self, event: PlayerEvent) {
        if let PlayerEvent::Seeked(duration) = event {
            self.last_position = Duration::seconds(duration.num_seconds());
            self.interpolated_duration = duration - self.last_position;
            self.last_sync = Instant::now();
        }
    }

    // remove unwrap in future
    pub async fn sync(
        &mut self,
        current_position: Duration,
        playback_status: PlaybackStatus,
    ) -> Result<(), String> {
        let tick_duration = Duration::from_std(self.last_sync.elapsed()).unwrap();

        if self.previous_playback_status == Some(PlaybackStatus::Playing) {
            self.interpolated_duration += tick_duration;
        }

        if current_position != self.last_position {
            self.last_position = current_position;
            self.interpolated_duration = Duration::zero();
        }

        self.last_sync = Instant::now();
        self.previous_playback_status = Some(playback_status);

        Ok(())
    }
}
