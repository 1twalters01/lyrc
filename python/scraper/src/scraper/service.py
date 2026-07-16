from scraper.models.track import Track
from scraper.models.lyrics import Lyrics
from scraper.providers.base import LyricsProvider
from scraper.providers.lrclib import LrcLibProvider

class LyricsService:
    def __init__(self, providers: dict[str, LyricsProvider]):
        self.providers = providers

    async def search(self, track: Track, provider_name: str) -> Lyrics | None:
        provider = self.providers.get(provider_name)
        if provider is None:
            raise ValueError(f"unknown provider: {provider_name}")

        return await provider.search(track)
