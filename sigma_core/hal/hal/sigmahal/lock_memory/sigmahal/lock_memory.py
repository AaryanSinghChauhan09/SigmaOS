# Generated method: SigmaHAL.lock_memory
import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any

class SigmaHAL:
    def lock_memory(self, address: int, size: int) -> bool:
        """USP: Hardware Memory Hardening. Prevents pages from being swapped to disk."""
        if not self._kernel32:
            return False
        try:
            return bool(self._virt_lock(address, size))
        except:
            return False