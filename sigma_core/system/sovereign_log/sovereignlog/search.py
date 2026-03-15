# Generated method: SovereignLog.search
import time
import threading
import json
import hashlib
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase

class SovereignLog:
    def search(self, module: str=None, action: str=None) -> List[Dict]:
        with self._lock:
            res = self.ledger
            if module:
                res = [r for r in res if r['module'] == module]
            if action:
                res = [r for r in res if r['action'] == action]
            return res