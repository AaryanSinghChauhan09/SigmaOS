# Generated method: SigmaSovereignScheduler.__init__
import time
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaJSON as _json_lib
    class json:
        loads = staticmethod(_json_lib.loads)
        dumps = staticmethod(_json_lib.dumps)
        load  = staticmethod(lambda f: _json_lib.loads(f.read()))
        dump  = staticmethod(lambda d, f, **kw: f.write(_json_lib.dumps(d)))
except Exception:
    import json
import os
from datetime import datetime, timedelta
from typing import List, Dict, Any, Optional

class SigmaSovereignScheduler:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.tasks = []
        self.habits = []
        self.focus_blocks = []
        self.stats = {'focus_protected_hrs': 0.0, 'tasks_auto_scheduled': 0, 'habit_hits': 0}