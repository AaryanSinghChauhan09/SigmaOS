# Generated method: ProcessSignal.level_confidence
import time
import threading
import collections
import math
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class ProcessSignal:
    @property
    def level_confidence(self) -> float:
        """Calculates forecast stability based on variance."""
        if not self.history:
            return 1.0
        avg = sum(self.history) / len(self.history)
        variance = sum(((x - avg) ** 2 for x in self.history)) / len(self.history)
        return max(0.5, 1.0 - math.sqrt(variance) / 100.0)