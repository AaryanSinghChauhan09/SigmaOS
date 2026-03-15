# Generated method: SigmaInitEngine.list_services
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaInitEngine:
    def list_services(self) -> List[str]:
        return [f"[{svc['status'].upper()}] {name} (PID: {svc['pid']}) latency={svc.get('latency_ms', 0)}ms" for name, svc in self._services.items()]