from abc import ABC, abstractmethod

from models.track import track
from models.lyrics import lyrics

class LyricsProvider(ABC):

    @abstractmethod
    async def search(self, track: Track) -> Lyrics | None:
        pass
