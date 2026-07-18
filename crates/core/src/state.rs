use std::time::Instant;

use chrono::Duration;
use mpris::{playback::PlaybackStatus, track::Track};
use subtitles::subtitles::SubtitleDocument;

pub struct AppState {
    pub track: Option<Track>,
    pub subtitles: SubtitleDocument,
    pub playback_state: PlaybackState,
    pub last_updated: Instant, // Change to something in Chrono?
    pub playback_speed: f64, // Add event for change in playback speed
    
    // other app state

}

pub struct PlaybackState {
    pub status: PlaybackStatus,
    pub position: Duration,
}
