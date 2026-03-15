"""
Auto-split from sigma_core\hal\hal.py — SigmaHAL.get_hardware_state
"""

import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any



class SigmaHAL:
    def get_hardware_state(self):
        gpu = self.get_gpu_telemetry()
        return {'platform': self.host_os, 'cpu_cores': self.cpu_count, 'ram_load': f'{self._get_ram_usage():.1f}%', 'cpu_load': f'{self._get_cpu_usage():.1f}%', 'gpu_vram': gpu['vram_load'], 'bus_status': 'LOCKED' if self._get_cpu_usage() > 90 else 'FLUID', 'kernel_hook': 'DIRECT_SYSCALL' if self._kernel32 else 'EMULATED', 'status': 'APEX_READY'}
