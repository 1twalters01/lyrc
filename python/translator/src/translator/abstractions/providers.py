from abc import ABC, abstractmethod
from typing import Generic
from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.options import OptionsT

class TranslationProvider(ABC, Generic[OptionsT]):
    @abstractmethod
    async def translate(
            self,
            cue: Cue,
            to_language: Language,
            options: OptionsT,
    ) -> list[Cue]:
        pass
