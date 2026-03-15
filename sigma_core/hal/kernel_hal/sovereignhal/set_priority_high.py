# Generated method: SovereignHAL.set_priority_high
import ctypes
import platform
import os
import sys
from typing import Any

class SovereignHAL:
    def set_priority_high(self):
        """USP: Apex Thread Locking."""
        if self.os_type == 'Windows' and self.k32:
            self.k32.SetPriorityClass(self.k32.GetCurrentProcess(), 128)
            return True
        return False