
import httpx
from scraper.models.lyrics import Lyrics, LyricsFormat, LyricsSource
from scraper.models.track import Track

BASE_URL = "https://lrclib.net"

class LrcLibProvider:
    def __init__(self, client: httpx.AsyncClient):
        self.client = client

    async def search(self, track: Track) -> Lyrics:
        params = {
            "track_name": track.title,
            "artist_name": track.artist,
            "album_name": track.album,
            "duration": int(track.duration.total_seconds()),
        }

        response = await self.client.get(
            f"{BASE_URL}/api/get",
            params=params,
        )

        if response.status_code == 404:
            return None

        response.raise_for_status()

        data = response.json()

        if data.get("syncedLyrics"):
            return Lyrics(
                content=data["syncedLyrics"],
                format=LyricsFormat.LRC,
                source=LyricsSource.LRCLIB,
            )

        if data.get("plainLyrics"):
            return Lyrics(
                content=data["plainLyrics"],
                format=LyricsFormat.TEXT,
                source=LyricsSource.LRCLIB,
            )

        return None
