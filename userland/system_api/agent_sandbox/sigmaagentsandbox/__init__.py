# Generated method: SigmaAgentSandbox.__init__
import os
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaFS as _sfs
    class shutil:
        copy = staticmethod(_sfs.copy); copy2 = staticmethod(_sfs.copy)
        copytree = staticmethod(lambda s,d,**kw: None)
        rmtree   = staticmethod(lambda p,**kw: None)
        move     = staticmethod(_sfs.copy)
except Exception:
    import shutil
import uuid
import time
import subprocess
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
from typing import Dict, Any, List

class SigmaAgentSandbox:
    def __init__(self, kernel):
        self.kernel = kernel
        self.base_dir = os.path.join(os.getcwd(), 'userland', 'silos', 'agents')
        self._active_silos: Dict[str, Dict] = {}
        if not os.path.exists(self.base_dir):
            os.makedirs(self.base_dir, exist_ok=True)