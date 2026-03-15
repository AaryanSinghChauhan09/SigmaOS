# Generated method: SigmaAdaptiveKernel.get_transition_history
import time
import threading
from enum import Enum, auto

class SigmaAdaptiveKernel:
    def get_transition_history(self, limit: int=20) -> list[dict]:
        """Returns the last N profile transitions for audit purposes."""
        return self._history[-limit:]