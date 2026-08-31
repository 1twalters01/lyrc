from dataclasses import dataclass
from aligner.abstractions.options import AlignmentOptions

@dataclass
class WhisperXOptions(AlignmentOptions):
    language_code: str
