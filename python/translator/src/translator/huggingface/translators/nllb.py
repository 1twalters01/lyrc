from transformers import PreTrainedModel, PreTrainedTokenizerBase
from translator.huggingface.translators.base import HuggingfaceFamilyTranslator
from translator.models.language import Language

class NLLBTranslator(HuggingfaceFamilyTranslator):
    @staticmethod
    def translate(
        model: PreTrainedModel,
        tokenizer: PreTrainedTokenizerBase,
        content: list[str],
        source_language: Language | None,
        target_language: Language,
    ) -> list[str]:
        if source_language is None:
            raise ValueError("NLLB requires a source language")

        source_code = source_language.flores_200
        target_code = target_language.flores_200

        tokenizer.src_lang = source_code

        inputs = tokenizer(
            content,
            return_tensors="pt"
            padding=True,
            truncation=True,
        )

        outputs = model.generate(
            **inputs,
            forced_bos_token_id=tokenizer.convert_tokens_to_ids(
                target_code
            ),
        )

        return tokenizer.batch_decode(
            outputs,
            skip_special_tokens=True,
        )
