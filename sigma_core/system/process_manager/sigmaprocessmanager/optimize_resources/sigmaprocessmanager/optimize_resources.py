# Generated method: SigmaProcessManager.optimize_resources
import time
import uuid
import random
import threading
from typing import Dict, List, Any, Optional
from enum import Enum
from dataclasses import dataclass, field

class SigmaProcessManager:
    def optimize_resources(self) -> Dict[str, Any]:
        terminated = []
        _freed_acc: float = 0.0
        with self._lock:
            for pid, proc in list(self._procs.items()):
                idle = proc.qos == QoSClass.BACKGROUND and proc.cpu_pct < 0.5
                if proc.state == ProcessState.ZOMBIE or idle:
                    _m_val = float(getattr(proc, 'mem_mb', 0.0))
                    _freed_acc = _freed_acc + _m_val
                    terminated.append(str(proc.name))
                    self._procs.pop(pid, None)
        return {'terminated': terminated, 'freed_mb': s_round(float(_freed_acc), 1)}