from abc import ABC, abstractmethod

from downloader.models.lyrics import Lyrics
from downloader.models.track import Track


class LyricsProvider(ABC):

    @abstractmethod
    async def search(
        self,
        track: Track,
        options: OptionsT,
    ) -> Lyrics | None:
        pass
