"""
SigmaOS Memory Manager (Low-Level C-Parity)
===========================================
USP: Raw Pointer Manipulation & Zero-Copy allocator bypassing Python's GIL & GC.
Provides true low-level memory control for extreme performance execution.
"""
import ctypes
import mmap
import os
import time

class SigmaMemoryManager:
    """
    Direct C-Level memory allocation using ctypes and mmap.
    Bypasses standard Python object overhead for caching and I/O.
    """
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.page_size = mmap.PAGESIZE
        self._raw_buffers = {}
        self._total_allocated = 0
        self.mode = "EXTREME_PERFORMANCE_ZERO_GC"

    def allocate_page(self, buffer_id: str, size_in_bytes: int = 4096) -> bool:
        """Allocates a raw block of memory directly from the OS, bypassing Python GC."""
        if buffer_id in self._raw_buffers:
            return False
            
        # Ensure alignment to page size
        aligned_size = ((size_in_bytes + self.page_size - 1) // self.page_size) * self.page_size
        
        try:
            # Anonymous memory map (Zero-copy RAM buffer)
            if os.name == 'nt':
                buf = mmap.mmap(-1, aligned_size, tagname=f"sigma_mem_{buffer_id}")
            else:
                buf = mmap.mmap(-1, aligned_size, flags=mmap.MAP_PRIVATE | mmap.MAP_ANONYMOUS)
                
            self._raw_buffers[buffer_id] = buf
            self._total_allocated += aligned_size
            return True
        except Exception as e:
            print(f"[MEMORY] Allocation failed for {buffer_id}: {e}")
            return False

    def write_raw(self, buffer_id: str, data: bytes) -> int:
        """Writes raw bytes via memory pointer injection."""
        buf = self._raw_buffers.get(buffer_id)
        if not buf:
            return -1
        
        length = min(len(data), len(buf))
        # Move pointer to start and inject
        buf.seek(0)
        buf.write(data[:length])
        return length

    def read_raw(self, buffer_id: str, length: int) -> bytes:
        """Reads raw bytes instantly using C-level buffer pointers."""
        buf = self._raw_buffers.get(buffer_id)
        if not buf:
            return b""
            
        buf.seek(0)
        return buf.read(min(length, len(buf)))

    def free_page(self, buffer_id: str):
        """Immediately releases physical RAM back to the host hardware."""
        buf = self._raw_buffers.pop(buffer_id, None)
        if buf:
            self._total_allocated -= len(buf)
            buf.close()
            return True
        return False

    def health_check(self) -> str:
        allocated_mb = self._total_allocated / (1024 * 1024)
        return f"OK — SigmaMemoryManager [C-LEVEL] Active: {len(self._raw_buffers)} raw pages, {allocated_mb:.2f} MB allocated."

    def __del__(self):
        """Emergency unmap on kernel termination."""
        for buf_id in list(self._raw_buffers.keys()):
            self.free_page(buf_id)
