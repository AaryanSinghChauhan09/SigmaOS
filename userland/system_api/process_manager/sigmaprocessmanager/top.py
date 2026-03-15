"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.top
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def top(self, n: int=10) -> list[dict]:
        """Return top-N processes by CPU usage."""
        return sorted(self.list_processes(), key=lambda p: p['cpu'], reverse=True)[:n]
