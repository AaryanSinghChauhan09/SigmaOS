# Generated method: SigmaMemoryManager.allocate_page
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    def allocate_page(self, buffer_id: str, size_in_bytes: int=4096) -> bool:
        """Allocates a raw block of memory directly from the OS, bypassing Python GC."""
        if buffer_id in self._raw_buffers:
            return False
        aligned_size = (size_in_bytes + self.page_size - 1) // self.page_size * self.page_size
        try:
            if os.name == 'nt':
                buf = mmap.mmap(-1, aligned_size, tagname=f'sigma_mem_{buffer_id}')
            else:
                buf = mmap.mmap(-1, aligned_size, flags=mmap.MAP_PRIVATE | mmap.MAP_ANONYMOUS)
            self._raw_buffers[buffer_id] = buf
            self._total_allocated += aligned_size
            return True
        except Exception as e:
            print(f'[MEMORY] Allocation failed for {buffer_id}: {e}')
            return False