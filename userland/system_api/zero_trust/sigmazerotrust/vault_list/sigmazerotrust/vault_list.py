# Generated method: SigmaZeroTrust.vault_list
import time
import uuid
import hashlib
import hmac
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaZeroTrust:
    def vault_list(self) -> list[str]:
        return list(self._vault.keys())