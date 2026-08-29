from abc import ABC, abstractmethod
from aligner.models.cue import Cue
from aligner.models.aligned_cue import AlignedCue

class Aligner(ABC):
    @abstractmethod
    def align_cues(self, lrc_content: list[Cue], audio_path: str) -> AlignedCue:
        pass
