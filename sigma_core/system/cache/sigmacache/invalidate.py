# Generated method: SigmaCache.invalidate
import time
import hashlib
import zlib
import sys
from typing import Any, Dict, Optional
from .interfaces import SigmaModuleBase

class SigmaCache:
    def invalidate(self, pattern: str):
        """Wildcard invalidation for batch updates."""
        to_del = [h for h, r in self.store.items() if pattern in r['key_ref']]
        for h in to_del:
            del self.store[h]
            self.stats['evictions'] += 1