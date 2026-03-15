"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.defer_to_green_window
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def defer_to_green_window(self, pid: str, reason: str='batch_job') -> dict:
        """
            Suspends BACKGROUND processes until the grid carbon intensity drops below
            threshold (green window). Resumes automatically when conditions improve.
            """
        proc = self._procs.get(pid)
        if proc is None:
            return {'error': f'PID {pid} not found.'}
        if proc.qos != QoSClass.BACKGROUND:
            return {'warning': f"'{proc.name}' is not BACKGROUND QoS; deferral skipped."}
        proc.state = ProcessState.DEFERRED
        self._carbon_deferred.append(pid)
        return {'pid': pid, 'name': proc.name, 'reason': reason, 'message': f"CarbonSched: '{proc.name}' deferred to next green window. Will resume when grid intensity < 200 gCO₂/kWh."}
