"""
Auto-split from userland\system_api\sigma_app_store.py — SigmaAppStore.get_catalog
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json



class SigmaAppStore:
    def get_catalog(self, category: Optional[str]=None) -> List[Dict]:
        """Returns the full sovereign app catalog, optionally filtered by category."""
        apps = self._catalog.values()
        if category:
            apps = [a for a in apps if a.category.lower() == category.lower()]
        return [a.to_dict() for a in apps]
