"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager._audit_event
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def _audit_event(self, event: str, pid: str, detail: str):
        self._audit.append({'ts': time.strftime('%Y-%m-%dT%H:%M:%S'), 'event': event, 'pid': pid, 'detail': detail})
