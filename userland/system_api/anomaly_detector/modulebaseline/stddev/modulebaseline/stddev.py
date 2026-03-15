# Generated method: ModuleBaseline.stddev
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional

class ModuleBaseline:
    @property
    def stddev(self) -> float:
        if self.n < 2:
            return 0.01
        return math.sqrt(max(0, self._m2 / (self.n - 1)))