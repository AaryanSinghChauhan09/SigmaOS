"""
SigmaOS Sovereign Ledger (v2.0 Apex)
=====================================
USP: Multi-Ring Event Logging with Cryptographic Integrity.
Sovereign alternative to: Syslog, ELK, and Splunk (local).
"""
import time
import threading
import json
import hashlib
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase

class SovereignLog(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.ledger: List[Dict[str, Any]] = []
        self._lock = threading.Lock()
        self.stats = {"commits": 0, "integrity_score": 100.0}

    def commit(self, module: str, action: str, context: Dict[str, Any]):
        """USP: Cryptographic Ledger Commitment."""
        with self._lock:
            ts = time.time()
            # Calculate rolling hash for integrity
            prev_hash = self.ledger[-1]["hash"] if self.ledger else "GENESIS"
            payload = f"{ts}{module}{action}{json.dumps(context)}{prev_hash}"
            entry_hash = hashlib.sha256(payload.encode()).hexdigest()
            
            entry = {
                "ts": ts,
                "module": module,
                "action": action,
                "context": context,
                "hash": entry_hash
            }
            self.ledger.append(entry)
            self.stats["commits"] += 1
            
            # Trim ledger to 5000 entries (Sovereign Resource Preservation)
            if len(self.ledger) > 5000:
                self.ledger.pop(0)

    def search(self, module: str = None, action: str = None) -> List[Dict]:
        with self._lock:
            res = self.ledger
            if module: res = [r for r in res if r["module"] == module]
            if action: res = [r for r in res if r["action"] == action]
            return res

    def verify_integrity(self) -> bool:
        """USP: Chain-of-Custody Verification."""
        with self._lock:
            if not self.ledger: return True
            for i in range(1, len(self.ledger)):
                prev = self.ledger[i-1]
                curr = self.ledger[i]
                payload = f"{curr['ts']}{curr['module']}{curr['action']}{json.dumps(curr['context'])}{prev['hash']}"
                if hashlib.sha256(payload.encode()).hexdigest() != curr["hash"]:
                     self.stats["integrity_score"] -= 10.0
                     return False
            return True

    def health_check(self) -> str:
        return f"OK — Commits: {self.stats['commits']} | Integrity: {self.stats['integrity_score']}%"
