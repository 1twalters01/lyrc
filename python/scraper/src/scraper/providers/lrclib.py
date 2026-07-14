import httpx
from urllib.parse import urlencode

from scraper.models.track import Track
from scraper.models.lyrics import Lyrics, LyricsFormat, LyricsSource

BASE_URL = "https://lrclib.net"

class LrcLibProvider:

    async def search(self, track: Track) -> Lyrics:
        params = {
            "track_name": track.track_name,
            "artist_name": track.artist_name,
            "album_name": track.album_name,
            "duration": int(track.duration.total_seconds()),
        }

        async with httpx.AsyncClient() as client:
            response = await client.get(
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
