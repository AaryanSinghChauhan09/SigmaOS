"""
SigmaOS HAL - CPU Telemetry Shard
=================================
Low-level CPU metrics and core affinity.
"""
import time
import ctypes
from ctypes import wintypes

class CPUShard:
    @staticmethod
    def get_cpu_usage(hal) -> float:
        if hal.host_os != "Windows" or not hasattr(hal, '_kernel32'): return 10.0
        
        def _get_times():
            idle, kernel, user = wintypes.FILETIME(), wintypes.FILETIME(), wintypes.FILETIME()
            hal._kernel32.GetSystemTimes(ctypes.byref(idle), ctypes.byref(kernel), ctypes.byref(user))
            def _ft_to_int(ft): return (ft.dwHighDateTime << 32) | ft.dwLowDateTime
            return _ft_to_int(idle), _ft_to_int(kernel), _ft_to_int(user)

        try:
            i1, k1, u1 = _get_times()
            time.sleep(0.01) 
            i2, k2, u2 = _get_times()
            idle_delta = i2 - i1
            total_delta = (k2 - k1) + (u2 - u1)
            if total_delta == 0: return 0.0
            return 100.0 * (1.0 - (idle_delta / total_delta))
        except:
            return 5.0

    @staticmethod
    def pin_to_cores(hal, mask: int = 1) -> bool:
        if not hasattr(hal, '_kernel32'): return False
        try:
            handle = hal._kernel32.GetCurrentProcess()
            return bool(hal._set_affinity(handle, mask))
        except:
            return False
