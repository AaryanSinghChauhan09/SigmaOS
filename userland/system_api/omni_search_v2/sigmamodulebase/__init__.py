# Generated method: SigmaModuleBase.__init__
import os
import sys
import time
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaHash as _sigma_hash
    class hashlib:
        @staticmethod
        def sha256(data=b''):
            class _H:
                def __init__(self, d): self._d = d if isinstance(d,bytes) else d.encode()
                def hexdigest(self): return _sigma_hash.fnv1a_hex(self._d)
                def digest(self): return _sigma_hash.fnv1a_64(self._d).to_bytes(8,'big')
                def update(self, d): self._d += d if isinstance(d,bytes) else d.encode()
            return _H(data)
        md5 = sha256; sha1 = sha256; sha3_256 = sha256; sha3_512 = sha256
except Exception:
    import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaModuleBase:
    def __init__(self, kernel):
        self.kernel = kernel