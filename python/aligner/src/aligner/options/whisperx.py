from dataclasses import dataclass
from aligner.options.base import AlignmentOptions

@dataclass
class WhisperXOptions(AlignmentOptions):
    language_code: str
