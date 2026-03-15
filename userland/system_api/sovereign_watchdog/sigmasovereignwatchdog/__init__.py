# Generated method: SigmaSovereignWatchdog.__init__
import os
import sys
import time
import threading
import platform
import subprocess
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignWatchdog:
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self.is_running = False
        self._heal_log: List[Dict[str, Any]] = []
        self._thresholds = {'cpu_warning_pct': 90, 'mem_warning_pct': 85, 'disk_warning_pct': 95}