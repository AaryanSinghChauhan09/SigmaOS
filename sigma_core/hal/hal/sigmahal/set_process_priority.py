"""
Auto-split from sigma_core\hal\hal.py — SigmaHAL.set_process_priority
"""

import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any



class SigmaHAL:
    def set_process_priority(self, level: str='High'):
        if self.host_os != 'Windows' or not self._kernel32:
            return False
        levels = {'Realtime': 256, 'High': 128, 'Above': 32768, 'Normal': 32, 'Below': 16384, 'Idle': 64}
        try:
            handle = self._kernel32.GetCurrentProcess()
            priority = levels.get(level, levels['High'])
            self._kernel32.SetPriorityClass(handle, priority)
            return True
        except:
            return False
