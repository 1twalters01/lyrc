from translator.models.cue import Cue
from translator.abstractions.providers import TranslationProvider
from translator.models.language import Language
from translator.abstractions.options import OptionsT

class TranslationService:
    def __init__(self, providers: dict[str, TranslationProvider]):
        self.providers = providers

    async def translate(
            self,
            provider_name: str,
            cues: list[Cue],
            to_language: Language,
            options: OptionsT,
    ) -> list[Cue]:
        provider = self.providers.get(provider_name)
        if provider is None:
            raise ValueError(f"unknown provider: {provider_name}")

        return provider.translate(cues, to_language, options)
        
