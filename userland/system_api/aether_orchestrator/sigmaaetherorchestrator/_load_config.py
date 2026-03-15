# Generated method: SigmaAetherOrchestrator._load_config
import json
from dataclasses import dataclass
from typing import Dict, List, Any, Optional

class SigmaAetherOrchestrator:
    def _load_config(self):
        self.routes = {'default': 'gemini', 'offline': 'llama_local', 'mesh': 'sovereign_nodes'}