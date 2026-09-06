from dataclasses import dataclass

@dataclass
class HuggingfaceLoadedModel:
    model_id: str
    tokenizer: PreTrainedTokenizerBase
    model: PreTrainedModel
