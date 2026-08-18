from dataclasses import dataclass
from datetime import timedelta

@dataclass
class Word:
    start: timedelta
    end: timedelta
    text: str

@dataclass
class AlignedCue:
    start: timedelta
    end: timedelta
    words: list[Word]
