# Generated method: SigmaSovereignRegistry.get_key
import os
import sys
import json
import hashlib
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignRegistry:
    def get_key(self, path: str, default: Any=None) -> Any:
        keys = path.split('/')
        curr = self._data
        for k in keys:
            if isinstance(curr, dict) and k in curr:
                curr = curr[k]
            else:
                return default
        return curr