import os

from alignment.providers.whisperx import WhisperXAligner
from .lrc_contents import LRC_CONTENTS

def test_whisperx_aligner():
    lrc_contents = LRC_CONTENTS
    audio_path = os.environ["WHISPERX_TEST_AUDIO"]
    language_code = "es"
    device = "cuda"

    if audio_path is None:
        pytest.skip("WHISPERX_TEST_AUDIO not set")

    aligner = WhisperXAligner()
    aligned_cues = aligner.align_cues(
        lrc_contents,
        audio_path,
        language_code,
        device
    )
