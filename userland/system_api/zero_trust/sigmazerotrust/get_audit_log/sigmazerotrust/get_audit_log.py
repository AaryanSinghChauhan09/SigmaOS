# Generated method: SigmaZeroTrust.get_audit_log
import time
import uuid
import hashlib
import hmac
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaZeroTrust:
    def get_audit_log(self, limit: int=30) -> list[dict]:
        return self._audit[-limit:]