# Generated method: SigmaSovereignClipboardV2.search
import os
import sys
import json
import time
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignClipboardV2:
    def search(self, query: str) -> List[Dict[str, Any]]:
        """Fuzzy-searches clipboard history by content or label."""
        q = query.lower()
        found: List[Dict[str, Any]] = []
        for item in self._history:
            if len(found) >= 10:
                break
            if q in str(item.get('content', '')).lower() or q in str(item.get('label', '')).lower():
                found.append(item)
        return found