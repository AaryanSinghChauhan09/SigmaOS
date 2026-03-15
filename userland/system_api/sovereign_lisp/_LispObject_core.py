# Generated class core: LispObject
from dataclasses import dataclass
from typing import List, Any

@dataclass
class LispObject:
    type: str
    value: Any
    marked: bool = False