use chrono::Duration;

use crate::{parser::SubtitleParser, subtitles::SubtitleDocument};

pub enum LrcError {
    MissingTagClosingBracket,
    InvalidTimestamp,
    InvalidTimestampMillisecondFormat,
    MissingColonSeparatorInTimestamp,
}

enum LrcLine {
    Metadata {
        key: String,
        value: String,
    },

    Lyric {
        timestamps: Vec<Duration>,
        text: String,
    },
    Empty,
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
    ) -> Result<LrcLine, LrcError> {
    }

    fn parse_lyric(
        &self,
        line: &str,
    ) -> Result<LrcLine, LrcError> {
        let mut remaining_line = line;
        let mut timestamps = Vec::new();

        while let Some(stripped_line) = remaining_line.strip_prefix('[') {
            let Some(end) = stripped_line.find(']') else {
                return Err(LrcError::MissingTagClosingBracket);
            };

            let tag = &stripped_line[..end];
            remaining_line = &stripped_line[end + 1..];

            match LrcParser::parse_timestamp(tag) {
                Ok(timestamp) => timestamps.push(timestamp),
                Err(lrc_error) => return Err(lrc_error),
            }
        }

        if timestamps.is_empty() {
            return Ok(ParsedLine::Empty);
        }

        Ok(LrcLine::Lyric {
            timestamps,
            text: remaining_line.to_owned(),
        })
    }

    fn parse_timestamp(input: &str) -> Result<Duration, LrcError> {
        let trimmed_input = input.trim_start_matches('[').trim_end_matches(']');

        let colon_idx = match trimmed_input.find(':') {
            Some(colon_idx) => colon_idx,
            None => return Err(LrcError::MissingColonSeparatorInTimestamp),
        };

        let minutes = match trimmed_input[..colon_idx].parse::<i64>() {
            Ok(minutes) => Duration::minutes(minutes),
            Err(_) => return Err(LrcError::InvalidTimestamp),
        };

        let  remainder = &trimmed_input[colon_idx + 1..];
        let (seconds, milliseconds) = match remainder.find('.') {
            Some(dot_idx) => {
                let seconds = match remainder[..dot_idx].parse::<i64>() {
                    Ok(seconds) => Duration::seconds(seconds),
                    Err(_) => return Err(LrcError::InvalidTimestamp),
                };

                let milliseconds_str = &remainder[dot_idx + 1..];
                let milliseconds = match milliseconds_str.len() {
                    2 => match milliseconds_str.parse::<i64>() {
                        Ok(milliseconds) => Duration::milliseconds(milliseconds * 10),
                        Err(_) => return Err(LrcError::InvalidTimestamp),
                    },
                    3 => match milliseconds_str.parse::<i64>() {
                        Ok(milliseconds) => Duration::milliseconds(milliseconds),
                        Err(_) => return Err(LrcError::InvalidTimestamp),
                    },
                    _ => return Err(LrcError::InvalidTimestampMillisecondFormat)
                };

                (seconds, milliseconds)
            },
            None => {
                let seconds = match remainder.parse::<i64>() {
                    Ok(seconds) => Duration::seconds(seconds),
                    Err(_) => return Err(LrcError::InvalidTimestamp),
                };
                let milliseconds = Duration::milliseconds(0);
                (seconds, milliseconds)
            }
        };

        Ok(minutes + seconds + milliseconds)
    }
}
