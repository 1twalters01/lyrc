from abc import ABC, abstractmethod
from transformers import PreTrainedModel, PreTrainedTokenizerBase

class HuggingfaceFamilyTranslator(ABC):
    @staticmethod
    @abstractmethod
    def translate(
        model: PreTrainedModel,
        tokenizer: PreTrainedTokenizerBase,
        content: list[str],
        source_language: Language | None,
        target_language: Language,
    ) -> list[str]:
        pass
