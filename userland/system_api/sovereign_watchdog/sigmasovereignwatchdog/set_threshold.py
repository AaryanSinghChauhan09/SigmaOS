# Generated method: SigmaSovereignWatchdog.set_threshold
import os
import sys
import time
import threading
import platform
import subprocess
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignWatchdog:
    def set_threshold(self, key: str, value: int):
        """Allows runtime tuning of watchdog thresholds."""
        if key in self._thresholds:
            self._thresholds[key] = value