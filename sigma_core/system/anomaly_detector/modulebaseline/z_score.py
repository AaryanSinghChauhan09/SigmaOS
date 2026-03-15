# Generated method: ModuleBaseline.z_score
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class ModuleBaseline:
    def z_score(self, value: float) -> float:
        s = self.stddev
        if s < 1e-09:
            return 0.0
        return abs(value - self.mean) / s