# Generated method: SigmaAdaptiveKernel.health_check
import time
import threading
from enum import Enum, auto

class SigmaAdaptiveKernel:
    def health_check(self) -> str:
        return f"OK — Profile: {self.current_profile.name}, Transitions: {self._transition_count}, Observer: {('running' if self._running else 'stopped')}"