# Generated method: SovereignBridge.vanguard_health_check
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def vanguard_health_check(self) -> bool:
        if self.emulated or not self._rust_lib:
            return True
        self._rust_lib.vanguard_health_check.restype = ctypes.c_uint8
        return self._rust_lib.vanguard_health_check() == 172