# Generated method: SigmaSemanticBus.register_intent_handler
from typing import Dict, Any, Callable
import json

class SigmaSemanticBus:
    def register_intent_handler(self, intent_name: str, handler: Callable):
        """USP: Apps can register high-level capabilities, not just syscalls."""
        self._intents[intent_name] = handler
        return f"SigmaBus: Registered '{intent_name}' as a semantic endpoint."