from dataclasses import dataclass
from downloader.abstractions.options import DownloadOptions

@dataclass
class MusixmatchOptions(DownloadOptions):
    duration_tolerance: int

    # Have these in the code instead?
    page_size: int
    page: int
