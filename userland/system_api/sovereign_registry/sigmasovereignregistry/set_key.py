# Generated method: SigmaSovereignRegistry.set_key
import os
import sys
import json
import hashlib
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignRegistry:
    def set_key(self, path: str, value: Any):
        keys = path.split('/')
        curr = self._data
        for i in range(len(keys) - 1):
            k = keys[i]
            if k not in curr:
                curr[k] = {}
            curr = curr[k]
        last_key = keys[len(keys) - 1]
        curr[last_key] = value
        self.save_registry()