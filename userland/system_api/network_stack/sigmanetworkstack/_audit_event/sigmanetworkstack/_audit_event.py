# Generated method: SigmaNetworkStack._audit_event
import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaNetworkStack:
    def _audit_event(self, event: str, target: str, detail: str=''):
        self._audit.append({'ts': time.strftime('%Y-%m-%dT%H:%M:%S'), 'event': event, 'target': target, 'detail': detail})