use crate::subtitles::SubtitleDocument;

pub trait SubtitleParser {
    type Error;

    fn parse(&self, input: &str) -> Result<SubtitleDocument, Self::Error>;
}
