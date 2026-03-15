# Generated method: ZKSync._save_vault
import os
import base64
import hashlib
import json
from typing import Dict

class ZKSync:
    def _save_vault(self):
        os.makedirs(os.path.dirname(self.vault_path), exist_ok=True)
        with open(self.vault_path, 'w') as f:
            json.dump(self.vault, f)