# Generated method: SigmaAppStore.audit_log
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json

class SigmaAppStore:
    def audit_log(self) -> List[str]:
        """Returns the immutable install/update/uninstall ledger."""
        return list(self._ledger)