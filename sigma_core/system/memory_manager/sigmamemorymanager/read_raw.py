# Generated method: SigmaMemoryManager.read_raw
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    def read_raw(self, buffer_id: str, length: int) -> bytes:
        """Reads raw bytes instantly using C-level buffer pointers."""
        buf = self._raw_buffers.get(buffer_id)
        if not buf:
            return b''
        buf.seek(0)
        return buf.read(min(length, len(buf)))