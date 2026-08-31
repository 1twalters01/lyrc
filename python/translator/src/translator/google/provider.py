from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.google.options import GoogleOptions

class GoogleTranslator(TranslationProvider[GoogleOptions]):
    def __init__(self):
        pass

    def translate(
            self,
            cues: [Cue],
            language: Language,
            options: GoogleOptions,
    ) -> list[Cue]:
        pass

