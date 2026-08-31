from dataclasses import dataclass
from typing import TypeVar

@dataclass
class AlignmentOptions:
    pass

OptionsT = TypeVar("OptionsT", bound=AlignmentOptions)
