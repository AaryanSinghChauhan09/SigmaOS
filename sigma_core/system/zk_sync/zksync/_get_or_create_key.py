# Generated method: ZKSync._get_or_create_key
import os
import base64
import hashlib
import json
from typing import Dict

class ZKSync:
    def _get_or_create_key(self) -> str:
        """Retrieves or generates a local master encryption key."""
        return hashlib.sha256(b'sigma_sovereign_key').hexdigest()