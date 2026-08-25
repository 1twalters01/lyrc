use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ElrcError {
    MissingTagClosingBracket,
    InvalidTimestamp,
    InvalidTimestampMillisecondFormat,
    MissingColonSeparatorInTimestamp,
    ContentAfterMetadataTag,
    InvalidMetadata,
    MissingMetadataSeparator,
}

// Make this better later - have line numbers for example
impl fmt::Display for ElrcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElrcError::MissingTagClosingBracket => {
                write!(f, "Missing tag closing bracket")
            }
            ElrcError::InvalidTimestamp => write!(f, "Invalid timestamp"),
            ElrcError::InvalidTimestampMillisecondFormat => {
                write!(f, "Invalid timestamp millisecond format")
            }
            ElrcError::MissingColonSeparatorInTimestamp => {
                write!(f, "Missing colon separator in timestamp")
            }
            ElrcError::ContentAfterMetadataTag => {
                write!(f, "Content after metadata tag")
            }
            ElrcError::InvalidMetadata => write!(f, "Invalid metadata"),
            ElrcError::MissingMetadataSeparator => {
                write!(f, "Missing metadata separator")
            }
        }
    }
}

impl Error for ElrcError {}
