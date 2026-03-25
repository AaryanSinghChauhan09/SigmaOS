# Generated method: SigmaGatewayAgent.__init__
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
import uuid
from typing import List, Dict, Any, Optional
from datetime import datetime

class SigmaGatewayAgent:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.bus = getattr(kernel, 'bus', None)
        self.registry = getattr(kernel, 'registry', {})
        self.context_memory = []
        self._stats = {'messages_bridged': 0, 'proactive_briefs_sent': 0, 'cli_commands_proxied': 0}
        if self.bus:
            self.bus.subscribe('chat.incoming', self.handle_incoming_chat_event)
            self.bus.subscribe('system.alert', self.handle_system_alert)