use chrono::Duration;

use crate::{parser::SubtitleParser, subtitles::SubtitleDocument};

pub enum LrcError {
    Error
}

enum LrcLine {
    Metadata {
        key: String,
        value: String,
    },

    Lyric {
        timestamps: Vec<Duration>,
        text: String,
    }
}

pub struct LrcParser;

impl SubtitleParser for LrcParser {
    type Error = LrcError;

    fn parse(&self, input: &str) -> Result<SubtitleDocument, Self::Error> {
        let mut doc = SubtitleDocument::default();

        for line in input.lines() {
            self.parse_line(line, &mut doc)?;
        }

        Ok(doc)
    }
}

impl LrcParser {
    fn parse_line(
        &self,
        line: &str,
        doc: &mut SubtitleDocument
    ) -> Result<LrcLine, LrcError> {
    }

    fn parse_metadata(
        &self,
        line: &str,
        doc: &mut SubtitleDocument
    ) -> Result<LrcLine, LrcError> {
    }

    fn parse_lyric(
        &self,
        line: &str,
        doc: &mut SubtitleDocument
    ) -> Result<LrcLine, LrcError> {
    }
}
