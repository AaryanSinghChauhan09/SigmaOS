# Generated method: SigmaInitEngine.status_all
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaInitEngine:
    def status_all(self) -> List[str]:
        lines = []
        for name, svc in self._services.items():
            icon = '●' if svc['status'] == 'running' else '○'
            pid_str = f"PID={svc['pid']}" if svc['pid'] else 'inactive'
            lines.append(f"  {icon} {name:<22} [{svc['status']:<8}] {pid_str}")
        return lines