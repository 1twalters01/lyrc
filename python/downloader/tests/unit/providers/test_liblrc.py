from datetime import timedelta

import httpx
import pytest
import respx
from downloader.models.track import Track
from downloader.service import LyricsService
from downloader.lrclib.provider import LrcLibDownloader


@pytest.mark.asyncio
@respx.mock
async def test_liblrc_search():
    respx.get(
        "https://lrclib.net/api/get",
        params={
            "track_name": "Porcelana",
            "artist_name": "Rosalía",
            "album_name": "Lux (Complete Works)",
            "duration": "248",
        },
    ).mock(
        return_value=httpx.Response(
            200,
            json={
                "syncedLyrics": "[00:10.00] Hello",
                "plainLyrics": "Hello",
            },
        )
    )

    track = Track(
        title="Porcelana",
        artist="Rosalía",
        album="Lux (Complete Works)",
        duration=timedelta(seconds=248)
    )

    async with httpx.AsyncClient() as client:
        provider = LyricsService({
            "lrclib": LrcLibDownloader(client),
        })
        lyrics = await provider.search(track, "lrclib")

        print(lyrics)

    assert lyrics is not None
    assert lyrics.content
