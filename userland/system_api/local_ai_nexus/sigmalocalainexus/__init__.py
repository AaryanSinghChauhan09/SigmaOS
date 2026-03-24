# Generated method: SigmaLocalAINexus.__init__
import socket
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
import time
from typing import Dict, Any, Optional

class SigmaLocalAINexus:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.local_nodes: Dict[str, str] = {'ollama_layer': 'http://localhost:11434', 'lm_studio_layer': 'http://localhost:1234'}
        self.active_node: str = 'sigma-quantized-native'
        self.weights_loaded: bool = False