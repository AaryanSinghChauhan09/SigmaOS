# Generated method: SigmaCache._prune_oldest
import time
import hashlib
import zlib
import sys
from typing import Any, Dict, Optional
from .interfaces import SigmaModuleBase

class SigmaCache:
    def _prune_oldest(self):
        h_list = list(self.store.keys())
        for i in range(len(h_list) // 10):
            del self.store[h_list[i]]
            self.stats['evictions'] += 1