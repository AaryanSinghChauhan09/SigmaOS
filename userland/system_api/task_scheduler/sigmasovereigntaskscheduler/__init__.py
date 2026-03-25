# Generated method: SigmaSovereignTaskScheduler.__init__
import os
import sys
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
import queue
from typing import Dict, List, Any, Optional, Callable
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignTaskScheduler:
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self.task_queue: queue.Queue = queue.Queue()
        self.is_running = False
        self._worker_thread: Optional[threading.Thread] = None
        self.stats = {'tasks_completed': 0, 'priority_shifts': 0}