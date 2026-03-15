# Generated method: SigmaHAL.health_check
import platform
import os
import ctypes
import time
from ctypes import wintypes
from typing import Dict, Any

class SigmaHAL:
    def health_check(self) -> str:
        state = self.get_hardware_state()
        disk = self.get_disk_health()
        return f"OK — HAL v2.5 | {state['cpu_cores']} Cores | RAM: {state['ram_load']} | CPU: {state['cpu_load']} | Disk Latency: {disk['read_latency_ms']}ms | GPU: {state['gpu_vram']}"