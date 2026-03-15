# Generated method: SigmaAnomalyDetector.register_module
import time
import math
import threading
import collections
import random
from typing import Dict, List, Any, Optional

class SigmaAnomalyDetector:
    def register_module(self, name: str):
        with self._lock:
            if name not in self._baselines:
                self._baselines[name] = {'latency_ms': ModuleBaseline(name), 'event_rate': ModuleBaseline(name), 'error_rate': ModuleBaseline(name), 'mem_usage_mb': ModuleBaseline(name), 'cpu_pressure': ModuleBaseline(name)}