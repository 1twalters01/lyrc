from downloader.models.track import Track
from downloader.models.lyrics import Lyrics, LyricsFormat, LyricsSource
from downloader.abstractions.providers import LyricsProvider

import httpx

import asyncio

class LyricsService:
    def __init__(self, providers: dict[str, LyricsProvider]):
        self.providers = providers

    async def ping(self, track: Track, provider_name: str):
        await asyncio.sleep(2)
        return Lyrics(
            content="Ping",
            format=LyricsFormat.TEXT,
            source=LyricsSource.SELF,
        )

    async def search(self, track: Track, provider_name: str) -> Lyrics | None:
        provider = self.providers.get(provider_name)
        if provider is None:
            raise ValueError(f"unknown provider: {provider_name}")

        return await provider.search(track)
