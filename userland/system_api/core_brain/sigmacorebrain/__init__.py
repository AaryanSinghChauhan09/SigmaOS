# Generated method: SigmaCoreBrain.__init__
from typing import Dict, List, Any
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

class SigmaCoreBrain:
    def __init__(self, kernel):
        self.kernel = kernel
        self._rules = {'Sovereignty': 'Always prefer local compute and open standards.', 'Independence': 'Avoid vendor-specific lock-in; use abstract adapters.', 'Zero_Trust': 'Verify all external data before ingestion into Sigma-FS.'}
        self._prompt_templates = {'Meta_OS': 'You are the core OS brain for SigmaOS. Current mode: {mode}. Mode config: {config}. Goal: {goal}. Steps: 1. Interpret. 2. Route to Adapters. 3. Synthesize.'}