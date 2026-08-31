from aligner.models.aligned_cue import AlignedCue
from aligner.models.cue import Cue
from aligner.abstractions.providers import AlignmentProvider
from aligner.abstractions.options import AlignmentOptions

class AlignmentService:
    def __init__(self, providers: dict[str, AlignmentProvider]):
        self.providers = providers

    def align_cues(
            self,
            provider_name: str,
            lrc_content: list[Cue],
            audio_path: str,
            options: AlignmentOptions,
    ) -> list[AlignedCue]:
        provider = self.providers.get(provider_name)
        if provider is None:
            raise ValueError(f"unknown provider: {provider_name}")

        return provider.align_cues(lrc_content, audio_path, options)
