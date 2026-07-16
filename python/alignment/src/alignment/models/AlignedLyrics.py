from dataclasses import dataclass
from datetime import timedelta

@dataclass
class Word:
    text: str
    start: timedelta
    end: timedelta

@dataclass
class AlignedLine:
    start: float
    end: float
    words: list[Word]
