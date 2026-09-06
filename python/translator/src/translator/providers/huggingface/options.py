from dataclasses import dataclass
from enum import Enum

from translator.abstractions.options import TranslationOptions
from translator.models.language import Language
from translator.providers.huggingface.translators.base import HuggingfaceFamilyTranslator
from translator.providers.huggingface.translators.nllb import NLLBTranslator
from translator.providers.huggingface.translators.m2m100 import M2M100Translator
from translator.providers.huggingface.translators.marian import MarianTranslator

@dataclass
class HuggingfaceOptions(TranslationOptions):
    source_language: Language | None = None
    model: HuggingfaceModel

@dataclass
class HuggingfaceModel:
    model_id: str
    family: HuggingfaceFamily

class HuggingfaceFamily(Enum):
    NLLB = NLLBTranslator
    M2M100 = M2M100Translator
    MARIAN = MarianTranslator
