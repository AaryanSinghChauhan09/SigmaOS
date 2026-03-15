# Generated method: SigmaZeroTrust._record_permission
import time
import uuid
import hashlib
import hmac
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaZeroTrust:
    def _record_permission(self, identity: str, resource: str, action: str, effect: str):
        chain_input = f"{self._chain_hash}{identity}{resource}{action}{effect}{time.strftime('%Y-%m-%dT%H:%M:%S')}"
        chain_hash = hashlib.sha256(chain_input.encode()).hexdigest()
        self._chain_hash = chain_hash
        grant = PermissionGrant(grant_id=str(uuid.uuid4())[:8], identity=identity, resource=resource, action=action, granted_by='PolicyEngine', timestamp=time.strftime('%Y-%m-%dT%H:%M:%S'), expiry_ts='', chain_hash=chain_hash)
        self._ledger.append(grant)