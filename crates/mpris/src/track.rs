use std::path::PathBuf;

use chrono::Duration;

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub album: Option<String>,
    pub url: Option<PathBuf>,
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    TrackChanged(Track),
    PositionChanged(Duration),
    PlaybackChanged(PlaybackStatus),
}

pub struct MprisEvents {
    stream: PropertyChangedStream<'static>,
}
