# Generated method: SovereignBridge.ipc_create_channel
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def ipc_create_channel(self, sender_pid: int, receiver_pid: int) -> int:
        if self.emulated or not self._c_lib:
            return hash((sender_pid, receiver_pid)) % 256
        self._c_lib.sigma_ipc_create_channel.argtypes = [ctypes.c_uint32, ctypes.c_uint32]
        self._c_lib.sigma_ipc_create_channel.restype = ctypes.c_int
        return self._c_lib.sigma_ipc_create_channel(sender_pid, receiver_pid)