import httpx
from downloader.models.lyrics import Lyrics, LyricsFormat, LyricsSource
from downloader.models.track import Track
from downloader.abstractions.providers import LyricsProvider
from downloader.providers.musixmatch.options import MusixmatchOptions

BASE_URL = "https://api.musixmatch.com/ws/1.1"

class MusixmatchDownloader(LyricsProvider[MusixmatchOptions]):
    def __init__(self, client: httpx.AsyncClient, api_key: str):
        self.client = client
        self.api_key = api_key

    async def search(
        self,
        track: Track,
        options: MusixmatchOptions,
    ) -> Lyrics | None:
        params = {
            "apikey": self.api_key,
            "q_track": track.title,
            "q_artist": track.artist,
            "page_size": options.page_size,
            "page": options.page,
        }

        response = await self.client.get(
            f"{BASE_URL}/track.search",
            params=params,
        )

        if response.status_code == 404:
            return None

        response.raise_for_status()
        data = response.json()
        tracks = data["message"]["body"]["track_list"]

        match = self._find_match(
            track,
            tracks,
            options.duration_tolerance,
        )

        if match is None:
            return None

        commontrack_id = match["commontrack_id"]

        lyrics = await self._get_subtitle(
            commontrack_id,
        )

        if lyrics is not None:
            return lyrics

        return await self._get_lyrics(
            commontrack_id,
        )

    def _find_match(
        self,
        track: Track,
        candidates: list[dict],
        duration_tolerance: int,
    ) -> dict | None:
        duration = int(track.duration.total_seconds())

        for entry in candidates:
            candidate = entry["track"]

            if track.album:
                if candidate["album_name"].casefold() != track.album.casefold():
                    continue

            if abs(candidate["track_length"] - duration) > duration_tolerance:
                continue

            return candidate

        return None

    async def _get_subtitle(
        self,
        commontrack_id: int,
    ) -> Lyrics | None:
        response = await self.client.get(
            f"{BASE_URL}/track.subtitle.get",
            params={
                "apikey": self.api_key,
                "commontrack_id": commontrack_id,
                "subtitle_format": "lrc",
            },
        )

        if response.status_code == 404:
            return None

        response.raise_for_status()

        subtitle = response.json()["message"]["body"]["subtitle"]["subtitle_body"]

        if not subtitle:
            return None

        return Lyrics(
            content=subtitle,
            format=LyricsFormat.LRC,
            source=LyricsSource.MUSIXMATCH,
        )

    async def _get_lyrics(
        self,
        commontrack_id: int,
    ) -> Lyrics | None:
        response = await self.client.get(
            f"{BASE_URL}/track.lyrics.get",
            params={
                "apikey": self.api_key,
                "commontrack_id": commontrack_id,
            },
        )

        if response.status_code == 404:
            return None

        response.raise_for_status()

        lyrics = response.json()["message"]["body"]["lyrics"]["lyrics_body"]

        if not lyrics:
            return None

        return Lyrics(
            content=lyrics,
            format=LyricsFormat.TEXT,
            source=LyricsSource.MUSIXMATCH,
        )
