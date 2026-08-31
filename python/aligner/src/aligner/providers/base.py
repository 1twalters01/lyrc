from abc import ABC, abstractmethod
from typing import Generic
from aligner.models.cue import Cue
from aligner.models.aligned_cue import AlignedCue
from aligner.options.base import OptionsT

class AlignmentProvider(ABC, Generic[OptionsT]):
    @abstractmethod
    def align_cues(
            self,
            lrc_content: list[Cue],
            audio_path: str,
            options: OptionsT,
    ) -> list[AlignedCue]:
        pass
