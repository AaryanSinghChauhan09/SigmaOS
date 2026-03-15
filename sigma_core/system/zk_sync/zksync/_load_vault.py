# Generated method: ZKSync._load_vault
import os
import base64
import hashlib
import json
from typing import Dict

class ZKSync:
    def _load_vault(self) -> Dict:
        if os.path.exists(self.vault_path):
            with open(self.vault_path, 'r') as f:
                return json.load(f)
        return {}