use chrono::Duration;

use crate::track::Track;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
    Unknown,
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
    PositionChanged(Duration),
    PlaybackChanged(PlaybackStatus),
}
