# Generated method: BloomFilter.__contains__
from typing import List, Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class BloomFilter:
    def __contains__(self, item):
        for i in range(self.hash_count):
            index = hash(f'{item}-{i}') % self.size
            if not self.bit_array & 1 << index:
                return False
        return True