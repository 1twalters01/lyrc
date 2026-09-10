from abc import ABC, abstractmethod
from typing import Generic
from downloader.models.lyrics import Lyrics
from downloader.models.track import Track
from downloader.abstractions.options import OptionsT


class LyricsProvider(ABC, Generic[OptionsT]):

    @abstractmethod
    async def search(
        self,
        track: Track,
        options: OptionsT,
    ) -> Lyrics | None:
        pass

