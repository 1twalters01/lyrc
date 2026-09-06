from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.deepl.options import DeepLOptions
import deepl

class DeepLTranslator(TranslationProvider[DeepLOptions]):
    def __init__(self, api_key: str):
        self.client = deepl.DeepLClient(api_key)

    def translate(
            self,
            cues: list[Cue],
            to_language: Language,
            options: DeepLOptions,
    ) -> list[Cue]:
        content = [cue.content for cue in cues]

        if options.source_language is not None:
            source_language = options.source_language.code_2.upper()
        else:
            source_language = None

        results = self.client.translate_text(
            content,
            source_lang=source_language,
            target_lang=to_language.code_2.upper(),
            formality=options.formality,
        )

        translated_cues: list[Cue] = []
        for cue, result in zip(cues, results):
            translated_cues.append(
                Cue(
                    start=cue.start,
                    end=cue.end,
                    content=result.text,
                )
            )

        return translated_cues
