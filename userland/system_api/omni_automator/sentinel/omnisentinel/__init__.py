# Generated method: OmniSentinel.__init__
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
from typing import Dict, Any

class OmniSentinel:
    def __init__(self, stats: dict, kernel=None, launch_preset_fn=None):
        self.stats = stats
        self.kernel = kernel
        self.launch_preset_fn = launch_preset_fn
        self._running = False
        self._thread: threading.Thread | None = None