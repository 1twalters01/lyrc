from dataclasses import dataclass
from typing import TypeVar

@dataclass
class TranslationOptions:
    pass

OptionsT = TypeVar("OptionsT", bound=TranslationOptions)
