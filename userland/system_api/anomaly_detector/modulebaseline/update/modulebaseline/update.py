# Generated method: ModuleBaseline.update
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional

class ModuleBaseline:
    def update(self, value: float):
        self.n = self.n + 1
        self.last = value
        self.history.append(value)
        delta = value - self._mean
        self._mean = self._mean + delta / self.n
        delta2 = value - self._mean
        self._m2 = self._m2 + delta * delta2
        z = self.z_score(value)
        if len(self.z_history) > 0:
            self.drift = z - self.z_history[-1]
        self.z_history.append(z)