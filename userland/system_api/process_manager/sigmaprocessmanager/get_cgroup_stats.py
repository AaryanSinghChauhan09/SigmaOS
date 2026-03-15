"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.get_cgroup_stats
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def get_cgroup_stats(self) -> dict:
        procs_by_cg: dict[str, list[str]] = {k: [] for k in self._cgroups}
        for proc in self._procs.values():
            if proc.cgroup in procs_by_cg:
                procs_by_cg[proc.cgroup].append(proc.name)
        return {'cgroups': {k: {'cpu_quota': cg.cpu_quota, 'mem_limit_mb': cg.mem_limit_mb, 'io_weight': cg.io_weight, 'procs': procs_by_cg.get(k, [])} for k, cg in self._cgroups.items()}}
