# Generated method: ZeroTrustValidator.validate_module
import hashlib
import json
import re
from sigma_core.system.interfaces import ISigmaModule, SigmaModuleBase

class ZeroTrustValidator:
    def validate_module(self, name: str, signature: str) -> bool:
        """No signature = No execution."""
        if signature in self._trusted_keys:
            print(f"[TRUST] Module '{name}' verified via crypt-sig.")
            return True
        print(f"[TRUST] REJECT: Module '{name}' lacks a valid Sovereign signature.")
        return False