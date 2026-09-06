from datetime import timedelta
import whisperx
import tempfile
from pathlib import Path

from aligner.models.cue import Cue
from aligner.models.aligned_cue import AlignedCue, Word
from aligner.abstractions.providers import AlignmentProvider
from aligner.aeneas.options import AeneasOptions

class AeneasAligner(AlignmentProvider[AeneasOptions]):
    def __init__(self):
        pass

    def align_cues(
        self,
        content: list[Cue | str],
        audio_path: str,
        options: AeneasOptions,
    ) -> list[AlignedCue | Cue]:
        language_code = options.language.code_2

        cues = [
            cue
            for cue in content
            if cue.content.strip()
        ]

        if not cues:
            raise ValueError("No content found")

        # Aeneas needs a txt file so using tempfile to fake it
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)

            text_path = temp_dir / "lyrics.txt"
            sync_map_path = temp_dir / "sync_map.json"

