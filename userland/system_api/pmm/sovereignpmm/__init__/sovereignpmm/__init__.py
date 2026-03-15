# Generated method: SovereignPMM.__init__


class SovereignPMM:
    def __init__(self, kernel, total_mb=16384):
        self.kernel = kernel
        self.block_size = 4096
        self.total_blocks = total_mb * 1024 * 1024 // self.block_size
        self.bitmap = bytearray(self.total_blocks // 8)
        self.used_blocks = 0
        self.reserve_region(0, 1024 * 1024)