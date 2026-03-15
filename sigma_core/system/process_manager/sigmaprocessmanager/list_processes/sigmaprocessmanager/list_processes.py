# Generated method: SigmaProcessManager.list_processes
import time
import uuid
import random
import threading
from typing import Dict, List, Any, Optional
from enum import Enum
from dataclasses import dataclass, field

class SigmaProcessManager:
    def list_processes(self) -> List[Dict[str, Any]]:
        return [{'pid': p.pid, 'name': p.name, 'cpu': p.cpu_pct, 'mem': p.mem_mb} for p in self._procs.values()]