import os

from .lrc_contents import LRC_CONTENTS
from aligner.service import AlignmentService
from aligner.whisperx.providers import WhisperXProvider
from aligner.whisperx.options import WhisperXOptions

def test_whisperx_aligner():
    audio_path = os.getenv("WHISPERX_TEST_AUDIO")
    if audio_path is None:
        pytest.skip("WHISPERX_TEST_AUDIO not set")

    lrc_contents = LRC_CONTENTS
    language_code = "es"
    device = "cuda"

    service = AlignmentService({
        "whisperx": WhisperXProvider(device=device),
    })
    options = WhisperXOptions(language_code=language_code)
    aligned_cues = service.align_cues(
        "whisperx",
        lrc_contents,
        audio_path,
        options=options
    )

    print(aligned_cues)
