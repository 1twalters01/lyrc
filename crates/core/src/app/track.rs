use subtitles::subtitles::SubtitleDocument;

use crate::{app::App, renderer::Renderer};

impl<R> App<R>
where
    R: Renderer,
{
    pub async fn update_track_and_subtitle_document_information(&mut self) {
        self.state.track = self.mpris.get_current_track().await.ok();
        self.state.subtitle_document = match self.state.track {
            Some(ref track) => match &track.get_lrc_file_path() {
                Some(lyrics_file_path) => {
                    SubtitleDocument::from_pathbuf(lyrics_file_path.clone()).ok()
                }
                None => None,
            },
            None => None,
        };
    }
}
