from dataclasses import dataclass
from translator.abstractions.options import TranslationOptions
from translator.models.language import Language

@dataclass
class ArgosOptions(TranslationOptions):
    original_language: Language
