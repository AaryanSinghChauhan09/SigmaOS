# Generated method: SigmaNetworkGuardian.__init__
import time
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaThread as _T, SigmaLock as _L
    class threading:
        Thread = _T; Lock = _L; RLock = _L; Event = _L
        @staticmethod
        def current_thread(): return None
        @staticmethod
        def active_count(): return 1
except Exception:
    import threading
from typing import Dict, List, Any

class SigmaNetworkGuardian:
    def __init__(self, kernel):
        self.kernel = kernel
        self._connections: List[NetworkConnection] = []
        self._lock = threading.Lock()
        self._sinkhole_hits = 0
        self._current_qos = 'Balanced'
        self._active = True