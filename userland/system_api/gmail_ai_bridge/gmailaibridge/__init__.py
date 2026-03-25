# Generated method: GmailAIBridge.__init__
import os
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

class GmailAIBridge:
    def __init__(self, kernel):
        self.kernel = kernel
        self.authenticated = False
        self.current_user = None
        self.stats = {'emails_triaged': 0, 'drafts_refined': 0, 'minutes_saved': 0}