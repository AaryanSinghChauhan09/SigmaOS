# Generated method: SovereignBridge.get_monotonic_ns
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def get_monotonic_ns(self) -> int:
        if self.emulated or not hasattr(self._c_lib, 'sigma_timer_ns'):
            import time
            return int(time.time_ns())
        self._c_lib.sigma_timer_ns.restype = ctypes.c_uint64
        return self._c_lib.sigma_timer_ns()