# Generated method: SigmaInitEngine.start_service
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaInitEngine:
    def start_service(self, name: str) -> Dict:
        if name not in self._services:
            self._services[name] = {'status': 'running', 'restart': 'no', 'type': 'user', 'pid': random.randint(1000, 9999)}
            return {'status': 'OK', 'message': f"[init] Service '{name}' registered and started."}
        svc = self._services[name]
        svc['status'] = 'running'
        svc['pid'] = random.randint(1000, 9999)
        return {'status': 'OK', 'message': f"[init] Service '{name}' started (PID {svc['pid']})."}