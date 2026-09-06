from dataclasses import dataclass
from translator.abstractions.options import TranslationOptions

@dataclass
class DeepLOptions(TranslationOptions):
    api_key: str
    source_language: str | None = None
    formality: str | None = None
