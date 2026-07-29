use std::time::Instant;

use mpris::{
    playback::{PlaybackStatus, PlayerEvent},
    track::Track,
};
use subtitles::subtitles::SubtitleDocument;

#[derive(Clone)]
pub struct AppState {
    pub track: Option<Track>,
    pub subtitle_document: Option<SubtitleDocument>,
    pub playback_state: PlaybackStatus,
    pub last_updated: Option<Instant>,

    /* Add event for change in playback speed */
    pub playback_speed: f64,
    /* other app state */
}

impl AppState {
    pub fn new() -> Self {
        Self {
            track: None,
            subtitle_document: None,
            playback_state: PlaybackStatus::Unknown,
            last_updated: None,
            playback_speed: 1f64,
        }
    }

    pub fn update(&mut self, event: &PlayerEvent) {
        match event {
            PlayerEvent::TrackChanged(track) => {
                self.track = Some(track.clone())
                // need to change subtitle document
            }
            PlayerEvent::PlaybackChanged(playback) => self.playback_state = playback.clone(),
            PlayerEvent::Seeked(_duration) => {}
        }
    }
}
