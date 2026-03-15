"""
Auto-split from userland\system_api\sigma_app_store.py — SigmaAppStore.search
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json



class SigmaAppStore:
    def search(self, query: str) -> List[Dict]:
        """Full-text search across app names, descriptions, and categories."""
        q = query.lower()
        results = [a.to_dict() for a in self._catalog.values() if q in a.name.lower() or q in a.description.lower() or q in a.category.lower()]
        return results
