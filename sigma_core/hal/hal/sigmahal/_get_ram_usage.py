"""
Auto-split from sigma_core\hal\hal.py — SigmaHAL._get_ram_usage
"""

import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any



class SigmaHAL:
    def _get_ram_usage(self) -> float:
        if self.host_os != 'Windows' or not self._kernel32:
            return 42.0
        stat = MEMORYSTATUSEX()
        stat.dwLength = ctypes.sizeof(stat)
        self._kernel32.GlobalMemoryStatusEx(ctypes.byref(stat))
        return float(stat.dwMemoryLoad)
