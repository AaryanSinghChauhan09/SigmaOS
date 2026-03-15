# Generated method: SigmaMemoryManager.__del__
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    def __del__(self):
        """Emergency unmap on kernel termination."""
        for buf_id in list(self._raw_buffers.keys()):
            self.free_page(buf_id)