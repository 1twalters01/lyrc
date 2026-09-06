from google.cloud import translate_v3

from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.providers.google.options import GoogleOptions

class GoogleTranslator(TranslationProvider[GoogleOptions]):
    def __init__(self):
        self.client = translate_v3.TranslationServiceClient()
        self.parent = f"projects/{PROJECT_ID}/locations/global"

    def translate(
            self,
            cues: list[Cue],
            to_language: Language,
            options: GoogleOptions,
    ) -> list[Cue]:
        content = [cue.content for cue in cues]

        if options.source_language is not None:
            response = self.client.translate_text(
                contents=content,
                parent=self.parent,
                mime_type="text/plain",
                source_language_code=options.source_language.code_2,
                target_language_code=to_language.code_2,
            )
        else:
            response = self.client.translate_text(
                contents=content,
                parent=self.parent,
                mime_type="text/plain",
                target_language_code=to_language.code_2,
            )

        translated_cues: list[Cue] = []
        for cue, translation in zip(cues, response.translations):
            translated_cues.append(
                Cue(
                    start=cue.start,
                    end=cue.end,
                    content=translation.translated_text,
                )
            )

        return translated_cues
