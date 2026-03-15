# Generated method: SigmaContextEngine.health_check
from typing import Dict, List, Any
import time

class SigmaContextEngine:
    def health_check(self) -> str:
        return f'OK — Active Context: {self._active_context}.'