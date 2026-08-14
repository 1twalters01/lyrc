use subtitles::subtitles::SubtitleDocument;
use synchronizer::traits::Synchronizer;

use crate::{app::App, renderer::Renderer};

impl<R, S> App<R, S>
where
    R: Renderer,
    S: Synchronizer,
{
    pub async fn update_track_and_subtitle_document_information(&mut self) {
        self.state.track = self.mpris.get_current_track().await.ok();
        self.state.subtitle_document = match self.state.track {
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
}
