"""
Auto-split from userland\system_api\sigma_std.py — SigmaSys.cpu_usage
"""

import sys
import os
import time
import json
import hashlib
import hmac
import urllib.request
import subprocess



class SigmaSys:
    @staticmethod
    def cpu_usage():
        if sys.platform == 'win32':
            try:
                import ctypes
                from ctypes import wintypes

                class FILETIME(ctypes.Structure):
                    _fields_ = [('dwLowDateTime', wintypes.DWORD), ('dwHighDateTime', wintypes.DWORD)]
                idleTime = FILETIME()
                kernelTime = FILETIME()
                userTime = FILETIME()
                if ctypes.windll.kernel32.GetSystemTimes(ctypes.byref(idleTime), ctypes.byref(kernelTime), ctypes.byref(userTime)):

                    def to_int(ft):
                        return (ft.dwHighDateTime << 32) + ft.dwLowDateTime
                    idle = to_int(idleTime)
                    kernel = to_int(kernelTime)
                    user = to_int(userTime)
                    total_sys = kernel - SigmaSys._last_kernel + (user - SigmaSys._last_user)
                    idle_diff = idle - SigmaSys._last_idle
                    SigmaSys._last_idle = idle
                    SigmaSys._last_kernel = kernel
                    SigmaSys._last_user = user
                    if total_sys > 0:
                        return float((total_sys - idle_diff) * 100.0 / total_sys)
            except Exception:
                pass
        return 10.0
