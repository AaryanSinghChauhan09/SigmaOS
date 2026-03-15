# Generated method: ZenithOrchestrator.get_all_credentials
import json
import time
import uuid
import base64
import random
import platform
import threading
from pathlib import Path
from typing import Dict, List, Optional
from cryptography.fernet import Fernet
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC

class ZenithOrchestrator:
    def get_all_credentials(self) -> Dict:
        if not self.vault_path.exists():
            return {}
        try:
            with open(self.vault_path, 'rb') as f:
                return json.loads(self._fernet.decrypt(f.read()).decode())
        except:
            return {}