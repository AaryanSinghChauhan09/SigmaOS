# Generated method: SigmaMemoryManager.write_raw
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    def write_raw(self, buffer_id: str, data: bytes) -> int:
        """Writes raw bytes via memory pointer injection."""
        buf = self._raw_buffers.get(buffer_id)
        if not buf:
            return -1
        length = min(len(data), len(buf))
        buf.seek(0)
        buf.write(data[:length])
        return length