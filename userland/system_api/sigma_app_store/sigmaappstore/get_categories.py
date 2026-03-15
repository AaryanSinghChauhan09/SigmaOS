"""
Auto-split from userland\system_api\sigma_app_store.py — SigmaAppStore.get_categories
"""

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import hashlib
import json



class SigmaAppStore:
    def get_categories(self) -> List[str]:
        return sorted(set((a.category for a in self._catalog.values())))
