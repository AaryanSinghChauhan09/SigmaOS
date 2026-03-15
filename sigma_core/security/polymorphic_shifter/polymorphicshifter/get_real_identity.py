# Generated method: PolymorphicShifter.get_real_identity
import random
import time
import threading
from typing import Dict, List

class PolymorphicShifter:
    def get_real_identity(self, alias: str) -> str:
        """Resolves a masked name back to the real sovereign process."""
        with self._lock:
            for pid, info in self.active_processes.items():
                if info['current_alias'] == alias:
                    return info['real_name']
        return 'UNKNOWN_PROCESS'