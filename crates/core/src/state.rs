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
    pub automatic_scroll_offset: u16,
    pub manual_scroll_offset: Option<u16>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            track: None,
            subtitle_document: None,
            playback_state: PlaybackStatus::Unknown,
            last_updated: None,
            playback_speed: 1f64,

            automatic_scroll_offset: 0,
            manual_scroll_offset: None,
        }
    }

    pub fn update(&mut self, event: &PlayerEvent) {
        match event {
            PlayerEvent::TrackChanged(track) => {
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
}
