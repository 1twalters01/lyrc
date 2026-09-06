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
        MarianTranslator._validate_langauges(
            tokenizer,
            source_language,
            target_language,
        )

        inputs = tokenizer(
            content,
            return_tensors="pt",
            padding=True,
            truncation=True,
        )

        outputs = model.generate(**inputs)

        return tokenizer.batch_decode(
            outputs,
            skip_special_tokens=True,
        )

    @staticmethod
    def _validate_langauges(
        tokenizer: PreTrainedTokenizerBase,
        source_language: Language | None,
        target_language: Language,
    ) -> None:
        source_code = getattr(tokenizer, "source_lang", None)
        target_code = getattr(tokenizer, "target_lang", None)

        if source_language is not None and source_code != source_language.code_2:
            raise ValueError(
                f"Marian model expects source language '{source_code}', "
                f"but '{source_language.code_2}' was provided"
            )

        if target_code != target_language.code_2:
            raise ValueError(
                f"Marian model translates to '{target_code}', "
                f"but '{target_language.code_2}' was requested"
            )
