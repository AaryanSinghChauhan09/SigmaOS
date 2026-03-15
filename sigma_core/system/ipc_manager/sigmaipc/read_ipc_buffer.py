# Generated method: SigmaIPC.read_ipc_buffer
import mmap
import os
import threading
from typing import Dict, List, Any

class SigmaIPC:
    def read_ipc_buffer(self, name: str, length: int) -> bytes:
        """USP: Sovereign Data Extraction. Reads memory at silicon-speed."""
        buf = self._shared_buffers.get(name)
        if not buf:
            return b''
        buf.seek(0)
        return buf.read(min(length, len(buf)))