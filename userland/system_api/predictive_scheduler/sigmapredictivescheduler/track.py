# Generated method: SigmaPredictiveScheduler.track
import time
import threading
import collections
import math
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaPredictiveScheduler:
    def track(self, pid: str, name: str):
        with self._lock:
            self._signals[pid] = ProcessSignal(name)