"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.move_to_cgroup
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def move_to_cgroup(self, pid: str, cgroup: str) -> dict:
        if cgroup not in self._cgroups:
            return {'error': f"cgroup '{cgroup}' does not exist."}
        proc = self._procs.get(pid)
        if proc is None:
            return {'error': f'PID {pid} not found.'}
        old = proc.cgroup
        proc.cgroup = cgroup
        return {'pid': pid, 'old': old, 'new': cgroup, 'message': f"ProcessMgr: '{proc.name}' moved to cgroup '{cgroup}'."}
