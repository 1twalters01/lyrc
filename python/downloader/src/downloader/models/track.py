from dataclasses import dataclass
from datetime import timedelta


@dataclass
class Track:
    title: str
    artist: str
    album: str
    duration: timedelta
