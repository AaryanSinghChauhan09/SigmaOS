# Generated method: NetworkConnection.__init__
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
from typing import Dict, List, Any

class NetworkConnection:
    def __init__(self, pid: str, dest_ip: str, dest_port: int, domain: str=''):
        self.pid = pid
        self.dest_ip = dest_ip
        self.dest_port = dest_port
        self.domain = domain
        self.bytes_sent = 0
        self.bytes_recv = 0
        self.status = 'ESTABLISHED'
        self.blocked = False