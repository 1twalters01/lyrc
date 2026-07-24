use std::time::Instant;

use chrono::Duration;
use mpris::{playback::{PlaybackStatus, PlayerEvent}, track::Track};
use subtitles::subtitles::SubtitleDocument;

pub struct AppState {
    pub track: Option<Track>,
    pub subtitles: Option<SubtitleDocument>,
    pub playback_state: PlaybackState,
    pub last_updated: Instant, // Change to something in Chrono?
    pub playback_speed: f64,   /* Add event for change in playback speed */

    /* other app state */
}

impl AppState {
    pub fn update(&self, event: &PlayerEvent) {
        match event {
            PlayerEvent::TrackChanged(track) => {
            },
            PlayerEvent::PlaybackChanged(playback) => {}
            PlayerEvent::Seeked(duration) => {}
        }
    }
}

pub struct PlaybackState {
    pub status: PlaybackStatus,
    pub position: Duration,
}
