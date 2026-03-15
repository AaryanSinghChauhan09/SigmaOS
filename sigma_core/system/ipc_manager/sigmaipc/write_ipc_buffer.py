# Generated method: SigmaIPC.write_ipc_buffer
import mmap
import os
import threading
from typing import Dict, List, Any

class SigmaIPC:
    def write_ipc_buffer(self, name: str, data: bytes) -> int:
        """USP: Sovereign Context Injection. Directly writes to raw bytes."""
        buf = self._shared_buffers.get(name)
        if not buf:
            return -1
        length = min(len(data), len(buf))
        buf.seek(0)
        buf.write(data[:length])
        return length