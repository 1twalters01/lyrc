from dataclasses import dataclass
from aligner.abstractions.options import AlignmentOptions

@dataclass
class AeneasOptions(AlignmentOptions):
    language_code: str
