# Generated method: SigmaSovereignRegistry.save_registry
import os
import sys
import json
import hashlib
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignRegistry:
    def save_registry(self):
        try:
            with open(self.registry_path, 'w') as f:
                json.dump(self._data, f, indent=4)
        except Exception as e:
            print(f'Registry Save Fail: {e}')