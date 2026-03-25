# Generated method: SigmaOfflineGuard.__init__
import socket
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaHash as _sigma_hash
    class hashlib:
        @staticmethod
        def sha256(data=b''):
            class _H:
                def __init__(self, d): self._d = d if isinstance(d,bytes) else d.encode()
                def hexdigest(self): return _sigma_hash.fnv1a_hex(self._d)
                def digest(self): return _sigma_hash.fnv1a_64(self._d).to_bytes(8,'big')
                def update(self, d): self._d += d if isinstance(d,bytes) else d.encode()
            return _H(data)
        md5 = sha256; sha1 = sha256; sha3_256 = sha256; sha3_512 = sha256
except Exception:
    import hashlib
import time

class SigmaOfflineGuard:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._independence_score = 100.0
        self._blocked_outbound = 0
        self._stats = {'local_models_active': True, 'external_telemetry_disabled': True, 'p2p_discovery_only': True, 'app_sovereignty_enforced': True}
        self._sovereign_userland_apps = ['pdf_forge', 'titan_capture', 'sigma_browser', 'sigma_studio', 'sigma_lab', 'sigma_data_pro', 'omni_converter', 'aether_orchestrator']