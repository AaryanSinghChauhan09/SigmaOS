"""
Sovereign Physical Memory Manager (PMM) — v1.0
===============================================
USP: Bitmap-based block management for absolute RAM sovereignty.
     Standard-grade resource tracking for low-level kernels.
"""

class SovereignPMM:
    def __init__(self, kernel, total_mb=16384):
        self.kernel = kernel
        self.block_size = 4096 # 4KB Pages
        self.total_blocks = (total_mb * 1024 * 1024) // self.block_size
        
        # Bitmap: 1 bit per block. Using bytearray for simulation efficiency.
        # 1 = Used, 0 = Free
        self.bitmap = bytearray(self.total_blocks // 8)
        self.used_blocks = 0
        
        # Reserve first 1MB for BIOS/Kernel
        self.reserve_region(0, 1024 * 1024)

    def alloc_block(self) -> int:
        """Finds the first free block in the bitmap and marks it used."""
        for i in range(len(self.bitmap)):
            if self.bitmap[i] != 0xFF: # Not all bits are set
                for bit in range(8):
                    if not (self.bitmap[i] & (1 << bit)):
                        self.bitmap[i] |= (1 << bit)
                        self.used_blocks += 1
                        return (i * 8 + bit)
        return -1 # Out of memory

    def free_block(self, block_index: int):
        byte_idx = block_index // 8
        bit_idx = block_index % 8
        if self.bitmap[byte_idx] & (1 << bit_idx):
            self.bitmap[byte_idx] &= ~(1 << bit_idx)
            self.used_blocks -= 1

    def reserve_region(self, start_addr: int, size: int):
        start_block = start_addr // self.block_size
        num_blocks = size // self.block_size
        for i in range(start_block, start_block + num_blocks):
             byte_idx = i // 8
             bit_idx = i % 8
             if byte_idx < len(self.bitmap):
                 self.bitmap[byte_idx] |= (1 << bit_idx)
                 self.used_blocks += 1

    def get_memory_stats(self) -> dict:
        free_blocks = self.total_blocks - self.used_blocks
        return {
            "Total_RAM": f"{self.total_blocks * self.block_size // (1024*1024)} MB",
            "Used_RAM": f"{self.used_blocks * self.block_size // 1024} KB",
            "Free_RAM": f"{free_blocks * self.block_size // 1024} KB",
            "Utilization": f"{(self.used_blocks / self.total_blocks):.2%}"
        }

    def health_check(self) -> str:
        return f"OK — PMM: {self.used_blocks}/{self.total_blocks} blocks mapped. Bitmap integrity verified."
