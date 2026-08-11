use chrono::Duration;

use crate::{
    formats::lrc::error::LrcError,
    subtitles::{SubtitleContent, SubtitleDocument, SubtitleMetadata},
    writer::SubtitleWriter,
};

pub struct LrcWriter;

impl SubtitleWriter for LrcWriter {
    type Error = LrcError;

    fn write(&self, subtitle_document: &SubtitleDocument) -> Result<String, Self::Error> {
        let mut file = String::new();

        Self::write_metadata(&subtitle_document.metadata, &mut file);
        Self::write_cues(subtitle_document, &mut file);

        Ok(file)
    }
}

impl LrcWriter {
    pub fn write_metadata(subtitle_metadata: &SubtitleMetadata, file: &mut String) {
        if let Some(album) = &subtitle_metadata.album {
            file.push_str(&format!("[al:{album}]\n"));
        }

        if let Some(title) = &subtitle_metadata.title {
            file.push_str(&format!("[ti:{title}]\n"));
        }

        if !subtitle_metadata.artists.is_empty() {
            file.push_str(&format!("[ar:{}]\n", subtitle_metadata.artists.join(", ")));
        }

        if !subtitle_metadata.languages.is_empty() {
            file.push_str(&format!("[la:{}]\n", subtitle_metadata.artists.join(", ")));
        }
    }

    fn write_cues(document: &SubtitleDocument, output: &mut String) {
        for cue in &document.cues {
            let timestamp = Self::format_timestamp(cue.start);
            let text = match &cue.content {
                SubtitleContent::Text(text) => text,
            };

            output.push_str(&format!("[{}]{}\n", timestamp, text));
        }
    }

    fn format_timestamp(duration: Duration) -> String {
        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds() % 60;
        let milliseconds = duration.num_milliseconds() % 1_000;

        format!("{minutes:02}:{seconds:02}.{milliseconds:03}")
    }
}
