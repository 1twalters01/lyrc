from dataclasses import dataclass
from datetime import timedelta

@dataclass
class Language:
    name: str
    native_name: str
    code_2: str
    code_3: str
