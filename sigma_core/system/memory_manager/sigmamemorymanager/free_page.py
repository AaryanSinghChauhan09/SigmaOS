# Generated method: SigmaMemoryManager.free_page
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    def free_page(self, buffer_id: str):
        """Immediately releases physical RAM back to the host hardware."""
        buf = self._raw_buffers.pop(buffer_id, None)
        if buf:
            self._total_allocated -= len(buf)
            buf.close()
            return True
        return False