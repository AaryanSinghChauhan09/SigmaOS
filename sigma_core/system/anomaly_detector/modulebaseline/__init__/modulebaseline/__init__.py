# Generated method: ModuleBaseline.__init__
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class ModuleBaseline:
    def __init__(self, name: str):
        self.name = name
        self.n = 0
        self._mean = 0.0
        self._m2 = 0.0
        self.history = collections.deque(maxlen=1000)
        self.z_history = collections.deque(maxlen=100)
        self.last = 0.0
        self.drift = 0.0
        self.anomaly_count = 0