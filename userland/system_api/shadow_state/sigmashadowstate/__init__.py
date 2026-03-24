# Generated method: SigmaShadowState.__init__
import time
import copy
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
from typing import Dict, Any

class SigmaShadowState:
    def __init__(self, kernel):
        self.kernel = kernel
        self._shadows: Dict[str, Any] = {}
        self._last_sync: Dict[str, float] = {}
        self._is_recovering = False