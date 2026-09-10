from dataclasses import dataclass
from typing import TypeVar

@dataclass
class DownloadOptions:
    pass

OptionsT = TypeVar("OptionsT", bound=DownloadOptions)
