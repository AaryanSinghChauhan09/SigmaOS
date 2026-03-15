# Generated method: SigmaCache._hash_key
import time
import hashlib
import zlib
import sys
from typing import Any, Dict, Optional
from .interfaces import SigmaModuleBase

class SigmaCache:
    def _hash_key(self, key: str) -> str:
        return hashlib.sha256(key.encode()).hexdigest()