# Generated method: ZKSync.deobfuscate_name
import os
import base64
import hashlib
import json
from typing import Dict

class ZKSync:
    def deobfuscate_name(self, obs_name: str) -> str:
        return self.vault.get(obs_name, 'UNKNOWN_FILE')