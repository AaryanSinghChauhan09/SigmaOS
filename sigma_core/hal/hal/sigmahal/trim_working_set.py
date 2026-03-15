"""
Auto-split from sigma_core\hal\hal.py — SigmaHAL.trim_working_set
"""

import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any



class SigmaHAL:
    def trim_working_set(self):
        if self.host_os != 'Windows' or not self._kernel32:
            return False
        try:
            handle = self._kernel32.GetCurrentProcess()
            self._kernel32.SetProcessWorkingSetSize(handle, -1, -1)
            return True
        except:
            return False
