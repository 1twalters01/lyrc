from translator.models.cue import Cue
from translator.abstractions.providers import TranslatorProvider

class Translator:
    def __init__(self, providers: dict[str, TranslatorProvider])
        self.providers = providers

    async def translate(
            self,
            cues: [Cue],
            language: Language,
            options: OptionsT,
    ) -> list[Cue]:
        provider = self.providers.get(provider_name)
        if provider is None:
            raise ValueError(f"unknown provider: {provider_name}")

        return provider.translate(cues, language, options)
        
