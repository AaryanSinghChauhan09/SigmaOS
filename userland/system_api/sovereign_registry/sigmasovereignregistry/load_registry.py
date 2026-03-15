# Generated method: SigmaSovereignRegistry.load_registry
import os
import sys
import json
import hashlib
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaSovereignRegistry:
    def load_registry(self):
        if os.path.exists(self.registry_path):
            try:
                with open(self.registry_path, 'r') as f:
                    self._data = json.load(f)
            except:
                self._data = {}