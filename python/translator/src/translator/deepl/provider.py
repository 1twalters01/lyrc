from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.deepl.options import DeepLOptions

class DeepLTranslator(TranslationProvider[DeepLOptions]):
    def __init__(self):
        pass

    def translate(
            self,
            cues: [Cue],
            language: Language,
            options: DeepLOptions,
    ) -> list[Cue]:
        pass

