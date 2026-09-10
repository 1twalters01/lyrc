from dataclasses import dataclass
from downloader.abstractions.options import DownloadOptions

@dataclass
class MusixmatchOptions(DownloadOptions):
    duration_tolerance: int
    page_size: int
    page: int
