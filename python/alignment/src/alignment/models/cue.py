from dataclasses import dataclass
from datetime import timedelta

@dataclass
class Cue:
    start: timedelta
    end: timedelta
    content: str
