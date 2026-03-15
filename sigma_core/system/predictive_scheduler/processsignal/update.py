# Generated method: ProcessSignal.update
import time
import threading
import collections
import math
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class ProcessSignal:
    def update(self, sample: float):
        """Standard Holt-Winters Update Step."""
        if self.level == 0.0:
            self.level = sample
            self.trend = 0.0
        else:
            last_level = self.level
            self.level = _ALPHA * sample + (1 - _ALPHA) * (self.level + self.trend)
            self.trend = _BETA * (self.level - last_level) + (1 - _BETA) * self.trend
        self.history.append(sample)