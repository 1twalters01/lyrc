from scraper.models.lyrics import (
    Lyrics,
    LyricsFormat,
    LyricsSource,
)


def test_lrc_lyrics_creation():
    lyrics = Lyrics(
        content="[00:01.00] Hello\n[00:01.90] World",
        format=LyricsFormat.LRC,
        source=LyricsSource.LRCLIB,
    )

    assert lyrics.format == LyricsFormat.LRC
    assert lyrics.source == LyricsSource.LRCLIB
    assert "[00:01.00]" in lyrics.content
    assert "[00:01.00] Hello\n[00:01.90] World"
