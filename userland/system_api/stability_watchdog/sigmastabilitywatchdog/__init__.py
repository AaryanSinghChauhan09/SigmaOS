# Generated method: SigmaStabilityWatchdog.__init__
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
import collections
from typing import Dict, List, Any

class SigmaStabilityWatchdog:
    def __init__(self, kernel):
        self.kernel = kernel
        self._latencies: Dict[str, collections.deque] = {}
        self._failures: Dict[str, int] = {}
        self._tripped_modules: Dict[str, str] = {}
        self._threshold_latency_p99 = 200.0
        self._threshold_fails = 4
        self._stop_event = threading.Event()
        self._monitor_thread = None
        if hasattr(self.kernel, 'bus') and self.kernel.bus:
            self.kernel.bus.subscribe('kad.pre_trip', lambda p: self._on_pre_trip(p))