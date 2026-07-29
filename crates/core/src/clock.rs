use std::time::Instant;

use chrono::Duration;
use mpris::playback::{PlaybackStatus, PlayerEvent};

#[derive(Clone)]
pub struct PlaybackClock {
    last_position: Duration,
    interpolated_duration: Duration,
    offset: Duration,
    last_sync: Instant,
    previous_playback_status: Option<PlaybackStatus>,
}

impl PlaybackClock {
    pub fn new(offset: Duration) -> Self {
        let last_position = Duration::nanoseconds(0);
        let interpolated_duration = Duration::nanoseconds(0);
        let last_sync = Instant::now();
        let previous_playback_status = None;

        Self {
            last_position,
            interpolated_duration,
            offset,
            last_sync,
            previous_playback_status,
        }
    }

    pub fn get_position(&self) -> Duration {
        self.last_position + self.interpolated_duration + self.offset
    }

    pub fn update(&mut self, event: PlayerEvent) {
        if let PlayerEvent::Seeked(duration) = event {
            self.last_position = Duration::seconds(duration.num_seconds());
            self.interpolated_duration = duration - self.last_position;
            self.last_sync = Instant::now();
        }
    }

    pub fn sync(
        &mut self,
        current_position: Duration,
        playback_status: PlaybackStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tick_duration = Duration::from_std(self.last_sync.elapsed())?;

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
