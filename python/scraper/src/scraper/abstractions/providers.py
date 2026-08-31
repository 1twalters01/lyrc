from abc import ABC, abstractmethod

from scraper.models.lyrics import Lyrics
from scraper.models.track import Track


class LyricsProvider(ABC):

    @abstractmethod
    async def search(self, track: Track) -> Lyrics | None:
        pass
