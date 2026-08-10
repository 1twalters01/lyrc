use std::{fmt::Display, time::Instant};

use mpris::{
    playback::{PlaybackStatus, PlayerEvent},
    track::Track,
};
use subtitles::subtitles::SubtitleDocument;

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Edit,
    Select,
}

impl Display for AppMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Edit => write!(f, "edit"),
            Self::Select => write!(f, "select"),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub track: Option<Track>,
    pub subtitle_document: Option<SubtitleDocument>,
    pub playback_state: PlaybackStatus,
    pub last_updated: Option<Instant>,

    /* Add event for change in playback speed */
    pub playback_speed: f64,

    /* other app state */
    pub quit: bool,
    pub automatic_scroll_offset: usize,
    pub selected_cue: Option<usize>,
    pub app_mode: AppMode,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            track: None,
            subtitle_document: None,
            playback_state: PlaybackStatus::Unknown,
            last_updated: None,
            playback_speed: 1f64,

            quit: false,
            automatic_scroll_offset: 0,
            selected_cue: None,
            app_mode: AppMode::Normal,
        }
    }

    pub fn update(&mut self, event: &PlayerEvent) {
        match event {
            PlayerEvent::TrackChanged(track) => {
                if self.track.as_ref() != Some(track) {
                    self.selected_cue = None;
                }
                self.track = Some(track.clone());

                self.subtitle_document = match self.track {
                    Some(ref track) => match &track.file_path {
                        Some(file_path) => {
                            let mut lyrics_path = file_path.to_path_buf();
                            lyrics_path.set_extension("lrc");
                            SubtitleDocument::from_pathbuf(lyrics_path).ok()
                        }
                        None => None,
                    },
                    None => None,
                };
            }
            PlayerEvent::PlaybackChanged(playback) => self.playback_state = playback.clone(),
            PlayerEvent::Seeked(_duration) => {}
        }
    }

    pub fn is_normal_mode(&self) -> bool {
        self.app_mode == AppMode::Normal
    }
    pub fn is_edit_mode(&self) -> bool {
        self.app_mode == AppMode::Edit
    }
}
