from ..interfaces.base_sovereign import SigmaModule
import time

class MemoryPool:
    def __init__(self, block_size=1024, count=100):
        super().__init__('MEM_BLOCK', count)
        self.block_size = block_size