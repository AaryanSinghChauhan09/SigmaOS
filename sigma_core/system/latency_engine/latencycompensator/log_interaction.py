# Generated method: LatencyCompensator.log_interaction
import time
from typing import List, Tuple

class LatencyCompensator:
    def log_interaction(self, x: float, y: float):
        """Logs user mouse/touch interaction for pattern analysis."""
        self.input_history.append((time.time(), x, y))
        if len(self.input_history) > 50:
            self.input_history.pop(0)
        if len(self.input_history) > 10:
            self._analyze_trajectory()