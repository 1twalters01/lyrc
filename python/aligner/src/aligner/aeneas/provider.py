from datetime import timedelta
import whisperx
from aligner.models.cue import Cue
from aligner.models.aligned_cue import AlignedCue, Word
from aligner.abstractions.providers import AlignmentProvider
from aligner.aeneas.options import AeneasOptions

class AeneasAligner(AlignmentProvider[AeneasOptions]):
    def __init__(self):
        pass

    def align_cues(
        self,
        lrc_content: list[Cue],
        audio_path: str,
        options: AeneasOptions,
    ) -> list[AlignedCue]:
        pass

