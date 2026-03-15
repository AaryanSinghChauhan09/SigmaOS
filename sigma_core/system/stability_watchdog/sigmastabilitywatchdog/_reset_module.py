# Generated method: SigmaStabilityWatchdog._reset_module
import time
import threading
import collections
from typing import Dict, List, Any

class SigmaStabilityWatchdog:
    def _reset_module(self, name: str):
        self._failures[name] = 0
        if name in self._tripped_modules:
            del self._tripped_modules[name]