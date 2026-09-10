from datetime import timedelta

import httpx
import pytest
from downloader.models.track import Track
from downloader.service import LyricsService
from downloader.providers.lrclib.provider import LrcLibDownloader
from downloader.providers.lrclib.options import LrcLibOptions


@pytest.mark.integration
@pytest.mark.asyncio
async def test_liblrc_search_api():
    track = Track(
        title="Porcelana",
        artist="Rosalía",
        album="Lux (Complete Works)",
        duration=timedelta(seconds=248)
    )

    options = LrcLibOptions()

    async with httpx.AsyncClient(timeout=10.0) as client:
        service = LyricsService({
            "lrclib": LrcLibDownloader(client),
        })
        lyrics = await service.search(track, "lrclib", options)

        print(lyrics)

    assert lyrics is not None
    assert lyrics.content
