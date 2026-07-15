from datetime import timedelta

from scraper.models.track import Track


def test_track_creation():
    track = Track(
        title="Porcelana",
        artist="Rosalía",
        album="Lux (Complete Works)",
        duration=timedelta(seconds=248)
    )

    assert track.title == "Porcelana"
    assert track.artist == "Rosalía"
    assert track.album == "Lux (Complete Works)"
    assert track.duration == timedelta(minutes=4, seconds=8)
