# Generated class core: VisualBlock
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

@dataclass
class VisualBlock:
    id: str
    type: str
    name: str
    inputs: Dict[str, Union[str, float]] = field(default_factory=dict)
    next_block_id: str = None