from datetime import timedelta
import whisperx
from alignment.models.cue import Cue
from alignment.models.aligned_cue import AlignedCue, Word
from alignment.providers.base import Aligner

class WhisperXAligner(Aligner):
    def align_cues(self, lrc_content: list[Cue], audio_path: str, language_code: str, device: str) -> list[AlignedCue]:
        segments = [
            {
                "start": cue.start.total_seconds(),
                "end": cue.end.total_seconds(),
                "text": cue.content.strip()
            }
            for cue in lrc_content
            if cue.content.strip()
        ]
        if not segments:
            raise ValueError("No content found")

        audio = whisperx.load_audio(audio_path)
        model_a, metadata = whisperx.load_align_model(language_code=language_code, device=device)
        result = whisperx.align(segments, model_a, metadata, audio, device, return_char_alignments=False)

        aligned_cues: List[AlignedCue] = []
        for index, segment in enumerate(result["segments"]):
            segment_start = timedelta(
                seconds=segments[index].get("start", segment["start"])
            )
            segment_end = timedelta(
                seconds=segments[index].get("end", segment["end"])
            )
            segment_words = segment.get("words", [])
            print(f'\n{segment["start"]}')
            print(f"\n{segment_start} -> {segment_end}")

            words: List[Word] = []
            for segment_word in segment_words:
                word_start = timedelta(seconds=segment_word.get("start", segment_start))
                word_end = timedelta(seconds=segment_word.get("end", segment_end))
                word_text = segment_word.get("word", "")
                print(f"    {word_start} -> {word_end}: {word_text}")
                
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
            )

        return aligned_cues
