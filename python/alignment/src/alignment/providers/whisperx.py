from datetime import timedelta
import whisperx
from alignment.models.cue import Cue
from alignment.models.aligned_cue import AlignedCue, Word




def align_cues(lrc_content: list[cue], audio_path: str, output_elrc_path: str, language: str, device: str) -> list[AlignedCue]:
segments = [
    {
        "start": cue.start.total_seconds(),
        "end": cue.end.total_seconds(),
        "text": cue.content.strip()
    }
    for cue in cues
    if cue.content.strip()
]
if not segments:
    raise ValueError("No content found")

audio = whisperx.load_audio(audio_path)
model_a, metadata = whisperx.load_align_model(language_code=language_code, device=device)
result = whisperx.align(segments, model_a, metadata, audio, device, return_char_alignments=False)

aligned_cues: List[AlignedCue] = []
for segment in result["segments"]:
    segment_start = segment["start"]
    segment_end = segment["end"]
    segment_words = segment.get("words", [])

    words: List[Word] = []
    for segment_word in segment_words:
        word_start = timedelta(seconds=segment_word.get("start", segment_start))
        word_end = timedelta(seconds=segment_word.get("end", segment_end))
        word_text = segment_word.get("word", "")
        
        words.append(
            Word(
                start=word_start,
                end=word_end,
                text=word_text,
            )
        )


    aligned_cues.append(
        AlignedCue(
            start=segment_start,
            end=segment_end,
            words=words
    )
