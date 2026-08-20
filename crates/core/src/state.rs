use std::time::Instant;

use mpris::{
    client::MprisClient,
    playback::{PlaybackStatus, PlayerEvent},
    track::Track,
};
use subtitles::subtitles::SubtitleDocument;

use crate::{history::EditHistory, mode::AppMode};

#[derive(Clone)]
pub struct AppState {
    pub track: Option<Track>,
    pub subtitle_document: Option<SubtitleDocument>,
    pub edit_history: EditHistory,
    pub playback_state: PlaybackStatus,
    pub last_updated: Option<Instant>,

    /* Add event for change in playback speed */
    pub playback_speed: f64,

    /* other app state */
    pub quit: bool,
    pub automatic_scroll_offset: usize,
    pub app_mode: AppMode,
    pub unsaved_changes: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            track: None,
            subtitle_document: None,
            edit_history: EditHistory::new(),
            playback_state: PlaybackStatus::Unknown,
            last_updated: None,
            playback_speed: 1f64,

            quit: false,
            automatic_scroll_offset: 0,
            app_mode: AppMode::Normal,
            unsaved_changes: false,
        }
    }

    pub async fn update(
        &mut self,
        mpris: &mut MprisClient,
        event: &PlayerEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            PlayerEvent::TrackChanged(track) => {
                if self.track.as_ref() != Some(track) {
                    self.app_mode = AppMode::Normal
                }
                self.track = Some(track.clone());

                self.subtitle_document = match self.track {
                    Some(ref track) => match &track.get_lrc_file_path() {
                        Some(lyrics_file_path) => {
                            SubtitleDocument::from_pathbuf(lyrics_file_path.clone()).ok()
                        }
                        None => None,
                    },
                    None => None,
                };
            }
            PlayerEvent::PlaybackChanged(playback) => {
                self.playback_state = playback.clone();
                if playback == &PlaybackStatus::Stopped {
                    let targets = Vec::from(["mpv", "cmus"]);
                    let player = MprisClient::choose_player(targets).await?;
                    *mpris = MprisClient::connect(&player).await?;
                    self.playback_state = mpris.get_playback_status().await?;
                }
            }
            PlayerEvent::Seeked(_duration) => {}
        }

        Ok(())
    }

    pub fn is_normal_mode(&self) -> bool {
        match self.app_mode {
            AppMode::Normal => true,
            _ => false,
        }
    }
    pub fn is_select_mode(&self) -> bool {
        match self.app_mode {
            AppMode::Select {
                cue_index: _,
                selected_cues: _,
            } => true,
            _ => false,
        }
    }
    pub fn is_edit_mode(&self) -> bool {
        match self.app_mode {
            AppMode::Edit {
                cue_index: _,
                selected_cues: _,
            } => true,
            _ => false,
        }
    }
}
