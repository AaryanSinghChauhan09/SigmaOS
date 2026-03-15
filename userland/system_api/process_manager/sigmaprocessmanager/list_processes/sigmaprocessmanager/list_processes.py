# Generated method: SigmaProcessManager.list_processes
import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaProcessManager:
    def list_processes(self, state: ProcessState | None=None) -> list[dict]:
        result = []
        for pid, proc in self._procs.items():
            if state is None or proc.state == state:
                result.append({'pid': pid, 'name': proc.name, 'qos': proc.qos.name, 'state': proc.state.value, 'cpu': proc.cpu_pct, 'mem': proc.mem_mb, 'nice': proc.nice, 'cgroup': proc.cgroup})
        return result