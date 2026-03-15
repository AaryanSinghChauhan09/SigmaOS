# Generated method: BloomFilter.add
from typing import List, Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class BloomFilter:
    def add(self, item):
        for i in range(self.hash_count):
            index = hash(f'{item}-{i}') % self.size
            self.bit_array |= 1 << index