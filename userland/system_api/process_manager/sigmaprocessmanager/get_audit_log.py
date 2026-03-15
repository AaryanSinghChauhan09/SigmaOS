"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.get_audit_log
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def get_audit_log(self, limit: int=30) -> list[dict]:
        return self._audit[-limit:]
