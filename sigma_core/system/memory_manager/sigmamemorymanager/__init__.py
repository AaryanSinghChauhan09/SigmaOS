# Generated method: SigmaMemoryManager.__init__
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.page_size = mmap.PAGESIZE
        self._raw_buffers = {}
        self._total_allocated = 0
        self.mode = 'EXTREME_PERFORMANCE_ZERO_GC'