use std::{error::Error, fmt};

#[derive(Debug)]
pub enum LrcError {
    MissingTagClosingBracket,
    InvalidTimestamp,
    InvalidTimestampMillisecondFormat,
    MissingColonSeparatorInTimestamp,
    ContentAfterMetadataTag,
    InvalidMetadata,
    MissingMetadataSeparator,
}

// Make this better later - have line numbers for example
impl fmt::Display for LrcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LrcError::MissingTagClosingBracket => write!(f, "Missing tag closing bracket"),
            LrcError::InvalidTimestamp => write!(f, "Invalid timestamp"),
            LrcError::InvalidTimestampMillisecondFormat => {
                write!(f, "Invalid timestamp millisecond format")
            }
            LrcError::MissingColonSeparatorInTimestamp => {
                write!(f, "Missing colon separator in timestamp")
            }
            LrcError::ContentAfterMetadataTag => write!(f, "Content after metadata tag"),
            LrcError::InvalidMetadata => write!(f, "Invalid metadata"),
            LrcError::MissingMetadataSeparator => write!(f, "Missing metadata separator"),
        }
    }
}

impl Error for LrcError {}
