# Generated method: SigmaSemanticBus.__init__
from typing import Dict, Any, Callable
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

class SigmaSemanticBus:
    def __init__(self, kernel):
        self.kernel = kernel
        self._intents: Dict[str, Callable] = {}
        self._provider_map = {'save_document': 'SigmaFS.atomic_write', 'send_message': 'AuraRelay.mesh_broadcast', 'encrypt_data': 'SovereignVault.vault_plus', 'translate_media': 'UniversalTranslator.relay', 'optimize_hardware': 'HardwareWarden.tune'}