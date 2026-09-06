from dataclasses import dataclass
from translator.abstractions.options import TranslationOptions
from translator.models.language import Language

@dataclass
class OllamaOptions(TranslationOptions):
    source_language: Language | None = None
    model: str
