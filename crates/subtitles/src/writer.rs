pub trait SubtitleWriter {
    type Error;

    fn write(&self, subtitle_document: &SubtitleDocument) -> Result<String, self::Error>;
}
