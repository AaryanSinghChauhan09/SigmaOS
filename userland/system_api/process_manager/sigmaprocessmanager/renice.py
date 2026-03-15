"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.renice
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def renice(self, pid: str, nice: int) -> dict:
        proc = self._procs.get(pid)
        if proc is None:
            return {'error': f'PID {pid} not found.'}
        old = proc.nice
        proc.nice = max(-20, min(19, nice))
        self._audit_event('renice', pid, f'{old} → {nice}')
        return {'pid': pid, 'old_nice': old, 'new_nice': proc.nice, 'message': f"ProcessMgr: '{proc.name}' nice {old} → {proc.nice}."}
