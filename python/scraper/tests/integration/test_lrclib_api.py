from datetime import timedelta

import httpx
import pytest
from scraper.models.track import Track
from scraper.service import LyricsService
from scraper.providers.lrclib import LrcLibProvider


@pytest.mark.integration
@pytest.mark.asyncio
async def test_liblrc_search_api():
    track = Track(
        title="Porcelana",
        artist="Rosalía",
        album="Lux (Complete Works)",
        duration=timedelta(seconds=248)
    )

    async with httpx.AsyncClient(timeout=10.0) as client:
        provider = LyricsService({
            "lrclib": LrcLibProvider(client),
        })
        lyrics = await provider.search(track, "lrclib")

        print(lyrics)

    assert lyrics is not None
    assert lyrics.content
