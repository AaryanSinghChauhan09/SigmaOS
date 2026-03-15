"""
Auto-split from userland\system_api\sigma_app_store.py — SigmaAppStore._ledger_write
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json



class SigmaAppStore:
    def _ledger_write(self, entry: str):
        """Append-only sovereign ledger for all app lifecycle events."""
        self._ledger.append(entry)
        if self.kernel:
            try:
                self.kernel.bus.emit('app_store.ledger', {'entry': entry})
            except Exception:
                pass
