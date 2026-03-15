# Generated method: ZKSync.obfuscate_file
import os
import base64
import hashlib
import json
from typing import Dict

class ZKSync:
    def obfuscate_file(self, content: str, original_name: str) -> tuple[str, str]:
        """Returns (obfuscated_name, encrypted_content)."""
        obs_name = hashlib.sha256((original_name + self.secret_key).encode()).hexdigest()[:16] + '.sig'
        enc_content = base64.b64encode(content.encode()).decode()
        self.vault[obs_name] = original_name
        self._save_vault()
        return (obs_name, enc_content)