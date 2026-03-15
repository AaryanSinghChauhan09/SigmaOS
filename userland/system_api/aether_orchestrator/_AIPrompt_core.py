# Generated class core: AIPrompt
import json
from dataclasses import dataclass
from typing import Dict, List, Any, Optional

@dataclass
class AIPrompt:
    user_id: str
    intent: str
    target_model: str
    payload: dict
    timestamp: float