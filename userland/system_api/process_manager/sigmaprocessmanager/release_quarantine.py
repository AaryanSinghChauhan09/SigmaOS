"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.release_quarantine
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def release_quarantine(self, pid: str) -> dict:
        if pid in self._quarantine:
            self._quarantine.remove(pid)
            proc = self._procs.get(pid)
            if proc:
                proc.state = ProcessState.RUNNING
            return {'status': 'Released', 'pid': pid, 'message': f'ProcessMgr: PID {pid} removed from quarantine.'}
        return {'error': f'PID {pid} not in quarantine.'}
