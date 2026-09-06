from transformers import PreTrainedModel, PreTrainedTokenizerBase
from translator.huggingface.translators.base import HuggingfaceFamilyTranslator
from translator.models.language import Language

class MarianTranslator(HuggingfaceFamilyTranslator):
    @staticmethod
    def translate(
        model: PreTrainedModel,
        tokenizer: PreTrainedTokenizerBase,
        content: list[str],
        source_language: Language | None,
        target_language: Language,
    ) -> list[str]:
        pass
