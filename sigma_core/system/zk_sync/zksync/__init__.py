# Generated method: ZKSync.__init__
import os
import base64
import hashlib
import json
from typing import Dict

class ZKSync:
    def __init__(self, kernel, vault_path: str='sigma_storage/zk_vault.json'):
        self.kernel = kernel
        self.vault_path = vault_path
        self.secret_key = self._get_or_create_key()
        self.vault = self._load_vault()