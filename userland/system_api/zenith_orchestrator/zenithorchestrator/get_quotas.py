# Generated method: ZenithOrchestrator.get_quotas
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
    def get_quotas(self, force_refresh=False) -> Dict:
        if not self.quotas or force_refresh:
            for node in self.nodes:
                name = node['name']
                used = random.randint(10, 90)
                self.quotas[name] = {'used': used, 'total': 100, 'percent': used, 'ts': time.time()}
            with open(self.quota_path, 'w') as f:
                json.dump(self.quotas, f)
        return self.quotas