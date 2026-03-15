# Generated method: SigmaAetherOrchestrator.__init__
import json
from dataclasses import dataclass
from typing import Dict, List, Any, Optional

class SigmaAetherOrchestrator:
    def __init__(self, kernel):
        self.kernel = kernel
        self.history: List[AIPrompt] = []
        self.active_session = None
        self._load_config()