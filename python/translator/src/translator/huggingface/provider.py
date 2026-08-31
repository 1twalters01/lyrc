from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.huggingface.options import HuggingfaceOptions

class HuggingfaceTranslator(TranslationProvider[HuggingfaceOptions]):
    def __init__(self):
        pass

    def translate(
            self,
            cues: [Cue],
            language: Language,
            options: HuggingfaceOptions,
    ) -> list[Cue]:
        pass

