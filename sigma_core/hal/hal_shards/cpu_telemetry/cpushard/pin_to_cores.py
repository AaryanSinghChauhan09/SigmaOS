# Generated method: CPUShard.pin_to_cores
import time
import ctypes
from ctypes import wintypes

class CPUShard:
    @staticmethod
    def pin_to_cores(hal, mask: int=1) -> bool:
        if not hasattr(hal, '_kernel32'):
            return False
        try:
            handle = hal._kernel32.GetCurrentProcess()
            return bool(hal._set_affinity(handle, mask))
        except:
            return False