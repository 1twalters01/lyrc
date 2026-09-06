from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.providers.ollama.options import OllamaOptions
from ollama import chat

class OllamaTranslator(TranslationProvider[OllamaOptions]):
    def __init__(self, api_key: str):
        pass

    def translate(
        self,
        cues: list[Cue],
        to_language: Language,
        options: OllamaOptions,
    ) -> list[Cue]:
        source_language = (
            options.source_language.name
            if options.source_language is not None
            else "the source lanugage"
        )

        content = "\n".join(
            f"{i}: {cue.content}"
            for i, cue in enumerate(cues)
        )

        response = chat(
            model=options.model,
            messages=[
                {
                    "role": "system",
                    "content": (
                        "You are a lyrics translator. "
                        "Translate the lyrics while preserving their "
                        "meaning and style."
                    ),
                },
                {
                    "role": "user",
                    "content": (
                        f"Translate these lyrics to {to_language.name}.\n"
                        "Return exactly one translation for each line, "
                        "in the same order.\n\n"
                        f"{content}"
                    ),
                }
            ],
        )

        translations = response.message.content.splitlines()

        if len(translations) != len(cues):
            raise ValueError(
                f"Ollama returned {len(translations)} translations "
                f"for {len(cues)} cues"
            )

        translated_cues: list[Cue] = []
        for cue, translation in zip(cues, translations):
            translated_cues.append(
                Cue(
                    start=cue.start,
                    end=cue.end,
                    content=translation,
                )
            )

        return translated_cues
