from abc import ABC, abstractmethod
from typing import Generic
from translator.models.cue import Cue
from translator.models.language import Language

class TranslationProvider(ABC, Generic[OptionsT]):
    @abstractmethod
    async def translate(
            self,
            cue: Cue,
            language: Language,
            options: OptionsT,
    ) -> list[Cue]:
        pass
