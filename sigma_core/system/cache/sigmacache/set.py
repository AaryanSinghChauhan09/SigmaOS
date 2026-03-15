# Generated method: SigmaCache.set
import time
import hashlib
import zlib
import sys
from typing import Any, Dict, Optional
from .interfaces import SigmaModuleBase

class SigmaCache:
    def set(self, key: str, data: Any, ttl: int=300):
        """USP: Memory-Gated sharding for zero-leak caching."""
        if len(self.store) >= self.max_shards:
            self._prune_oldest()
        h = self._hash_key(key)
        compressed = False
        if isinstance(data, str) and len(data) > 1024:
            orig_size = sys.getsizeof(data)
            data = zlib.compress(data.encode('utf-8'))
            compressed = True
            self.stats['compression_savings_kb'] += (orig_size - sys.getsizeof(data)) / 1024.0
        self.store[h] = {'data': data, 'expiry': time.time() + ttl, 'key_ref': key, 'compressed': compressed}