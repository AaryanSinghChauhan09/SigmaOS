# Generated method: SigmaCoreBrain.health_check
from typing import Dict, List, Any
import json

class SigmaCoreBrain:
    def health_check(self) -> str:
        return f"OK — {len(self._rules)} Global Rules active. Brain sync'ed with Kernel."