from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.argos.options import ArgosOptions

class ArgosTranslator(TranslationProvider[ArgosOptions]):
    def __init__(self):
        pass

    def translate(
            self,
            cues: [Cue],
            language: Language,
            options: ArgosOptions,
    ) -> list[Cue]:
        pass

