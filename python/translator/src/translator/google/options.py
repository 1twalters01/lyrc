from dataclasses import dataclass
from translator.abstractions.options import TranslationOptions

@dataclass
class GoogleOptions(TranslationOptions):
    source_language: Language | None = None,
