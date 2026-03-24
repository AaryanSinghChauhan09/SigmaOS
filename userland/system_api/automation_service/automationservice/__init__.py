# Generated method: AutomationService.__init__
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

class AutomationService:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.port = 9999
        self.total_commands_dispatched = 0
        self.last_command_ts = 0.0
        self.secure_mode = True
        self.authorized_keys = ['0xAPEX', '0xSIGMA_CORE']
        self.active_webhooks: Dict[str, str] = {}