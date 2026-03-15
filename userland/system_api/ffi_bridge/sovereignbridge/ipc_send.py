# Generated method: SovereignBridge.ipc_send
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def ipc_send(self, channel_id: int, data: bytes, sender_pid: int) -> bool:
        if self.emulated:
            return True
        self._c_lib.sigma_ipc_send.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_uint16, ctypes.c_uint32]
        self._c_lib.sigma_ipc_send.restype = ctypes.c_int
        res = self._c_lib.sigma_ipc_send(channel_id, data, len(data), sender_pid)
        return res == 0