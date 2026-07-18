use crate::subtitles::SubtitleDocument;

pub trait SubtitleWriter {
    type Error;

    fn write(&self, subtitle_document: &SubtitleDocument) -> Result<String, Self::Error>;
}
