# Generated method: SigmaHAL._get_cpu_usage
import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any

class SigmaHAL:
    def _get_cpu_usage(self) -> float:
        if self.host_os != 'Windows' or not self._kernel32:
            return 10.0

        def _get_times():
            idle, kernel, user = (wintypes.FILETIME(), wintypes.FILETIME(), wintypes.FILETIME())
            self._kernel32.GetSystemTimes(ctypes.byref(idle), ctypes.byref(kernel), ctypes.byref(user))

            def _ft_to_int(ft):
                return ft.dwHighDateTime << 32 | ft.dwLowDateTime
            return (_ft_to_int(idle), _ft_to_int(kernel), _ft_to_int(user))
        try:
            i1, k1, u1 = _get_times()
            time.sleep(0.01)
            i2, k2, u2 = _get_times()
            idle_delta = i2 - i1
            total_delta = k2 - k1 + (u2 - u1)
            if total_delta == 0:
                return 0.0
            return 100.0 * (1.0 - idle_delta / total_delta)
        except:
            return 5.0