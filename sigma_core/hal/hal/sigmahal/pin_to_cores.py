"""
Auto-split from sigma_core\hal\hal.py — SigmaHAL.pin_to_cores
"""

import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any



class SigmaHAL:
    def pin_to_cores(self, mask: int=1) -> bool:
        """USP: Hard Core Affinity. Eliminates context-switch jitter by pinning to specific silicon."""
        if not self._kernel32:
            return False
        try:
            handle = self._kernel32.GetCurrentProcess()
            return bool(self._set_affinity(handle, mask))
        except:
            return False
