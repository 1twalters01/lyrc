from dataclasses import dataclass
from enum import Enum

class LyricsFormat(str, Enum):
    LRC = "lrc"
    TEXT = "text"

class LyricsSource(str, Enum):
    LRCLIB = "lrclib"

@dataclass
class Lyrics:
    content: str
    format: LyricsFormat
    source: LyricsSource
