use chrono::Duration;

use crate::track::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
    Unknown,
}

impl PlaybackStatus {
    pub fn parse(status: &str) -> Self {
        match status {
            "Playing" => PlaybackStatus::Playing,
            "Paused" => PlaybackStatus::Paused,
            "Stopped" => PlaybackStatus::Stopped,
            _ => PlaybackStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlaybackCommand {
    Play,
    Pause,
    Toggle,
    Next,
    Previous,
    Seek(Duration),
    SetPosition(Duration),
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    TrackChanged(Track),
    PlaybackChanged(PlaybackStatus),
    Seeked(Duration),
}
