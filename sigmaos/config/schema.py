"""
SigmaOS Unified Configuration Schema
Replaces scattered YAML/JSON parsing with strict dataclasses.
"""
from dataclasses import dataclass, field
from typing import Dict, Any, List
import json
import os

@dataclass
class SigmaProfile:
    username: str = "sigma_admin"
    theme: str = "dark"
    active_agent_persona: str = "general"
    feature_flags: Dict[str, bool] = field(default_factory=lambda: {
        "enable_gaming_hub": False,
        "enable_legal_studio": False
    })
    
    def save(self, filepath: str):
        with open(filepath, 'w') as f:
            json.dump(self.__dict__, f, indent=4)

    @classmethod
    def load(cls, filepath: str) -> 'SigmaProfile':
        if not os.path.exists(filepath):
            return cls()
        with open(filepath, 'r') as f:
            data = json.load(f)
            return cls(**data)
