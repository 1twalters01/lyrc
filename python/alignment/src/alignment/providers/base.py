from abc import ABC, abstractmethod
from alignment.models.cue import Cue
from alignment.models.aligned_cue import AlignedCue

class Aligner(ABC):
    @abstractmethod
    def align_cues(self, lrc_content: list[Cue], audio_path: str) -> AlignedCue:
        pass
