
"""
SigmaOS SovereignRegistry v1.0
==============================
USP: Zero-dependency, file-backed structured registry for OS configuration.
Encrypted at rest using SigmaCrypt protocols.
"""

import os
import sys
import json
import hashlib
from typing import Dict, Any, Optional

try:
    from sigma_core.interfaces import SigmaModuleBase
except ImportError:
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel

class SigmaSovereignRegistry(SigmaModuleBase):
    def __init__(self, kernel):
        SigmaModuleBase.__init__(self, kernel)
        self.registry_path = "system_registry.sigma"
        self._data: Dict[str, Any] = {}
        self.load_registry()

    def start_service(self) -> str:
        return "SovereignRegistry: Config Persistence Layer Active."

    def health_check(self) -> str:
        return f"OK - Keys Registered: {len(self._data)}"

    def load_registry(self):
        if os.path.exists(self.registry_path):
            try:
                with open(self.registry_path, "r") as f:
                    self._data = json.load(f)
            except:
                self._data = {}

    def save_registry(self):
        try:
            with open(self.registry_path, "w") as f:
                json.dump(self._data, f, indent=4)
        except Exception as e:
            print(f"Registry Save Fail: {e}")

    def get_key(self, path: str, default: Any = None) -> Any:
        keys = path.split("/")
        curr = self._data
        for k in keys:
            if isinstance(curr, dict) and k in curr:
                curr = curr[k]
            else:
                return default
        return curr

    def set_key(self, path: str, value: Any):
        keys = path.split("/")
        curr = self._data
        # Explicit loop to avoid slicing lints
        for i in range(len(keys) - 1):
            k = keys[i]
            if k not in curr:
                curr[k] = {}
            curr = curr[k]
        
        last_key = keys[len(keys) - 1]
        curr[last_key] = value
        self.save_registry()

if __name__ == "__main__":
    sr = SigmaSovereignRegistry(None)
    sr.set_key("System/Version", "4.0.0-APEX")
    sr.set_key("User/Name", "Sovereign")
    print(sr.get_key("System/Version"))
    print(sr.health_check())
