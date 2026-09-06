from transformers import AutoTokenizer, AutoModelForSeq2SeqLM, PreTrainedModel, PreTrainedTokenizerBase

from translator.models.cue import Cue
from translator.models.language import Language
from translator.abstractions.providers import TranslationProvider
from translator.huggingface.options import HuggingfaceOptions
from translator.huggingface.model import HuggingfaceLoadedModel

class HuggingfaceTranslator(TranslationProvider[HuggingfaceOptions]):
    def __init__(self):
        self.loaded_model: HuggingfaceLoadedModel | None = None

    def _get_model(
        self,
        model_id: str
    ) -> HuggingfaceLoadedModel:
        if self.loaded_model is None or self.loaded_model.model_name != model_id:
            tokenizer = AutoTokenizer.from_pretrained(model_id)
            model = AutoModelForSeq2SeqLM.from_pretrained(model_id)

            self.loaded_model = HuggingfaceLoadedModel(
                model_id=model_id,
                tokenizer=tokenizer,
                model=model,
            )

        return self.loaded_model

    def translate(
            self,
            cues: list[Cue],
            to_language: Language,
            options: HuggingfaceOptions,
    ) -> list[Cue]:
        loaded_model: HuggingfaceLoadedModel = self._get_model(options.model.model_id)

        content = [cue.content for cue in cues]

        translations = options.model.family.value.translate(
            model=loaded_model.model,
            tokenizer=loaded_model.tokenizer,
            content=content,
            source_language=options.source_language,
            target_language=to_language,
        )

        translated_cues: list[Cue] = []
        for cue, translation in zip(cues, translations):
            translated_cues.append(
                Cue(
                    start=cue.start,
                    end=cue.end,
                    content=translation,
                )
            )

        return translated_cues
