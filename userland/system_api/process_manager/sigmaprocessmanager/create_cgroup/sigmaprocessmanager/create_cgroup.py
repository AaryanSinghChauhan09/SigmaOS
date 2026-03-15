# Generated method: SigmaProcessManager.create_cgroup
import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaProcessManager:
    def create_cgroup(self, name: str, cpu_quota: float, mem_mb: float, io_weight: int=500) -> dict:
        cg = CGroup(name, cpu_quota, mem_mb, io_weight)
        self._cgroups[name] = cg
        return {'cgroup': name, 'cpu_quota': cpu_quota, 'mem_limit': mem_mb, 'io_weight': io_weight, 'message': f"cgroup v2: '{name}' created (CPU≤{cpu_quota}%, RAM≤{mem_mb}MB)."}