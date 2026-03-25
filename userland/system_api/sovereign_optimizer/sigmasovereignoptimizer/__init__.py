# Generated method: SigmaSovereignOptimizer.__init__
import os
import sys
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
import platform
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignOptimizer:
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self.stats = {'purged_mb': 0.0, 'optimizations': 0}