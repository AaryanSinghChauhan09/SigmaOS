# Generated method: SigmaInitEngine.stop_service
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaInitEngine:
    def stop_service(self, name: str) -> Dict:
        if name not in self._services:
            return {'status': 'ERR', 'message': f"[init] Service '{name}' not found."}
        self._services[name]['status'] = 'stopped'
        self._services[name]['pid'] = None
        return {'status': 'OK', 'message': f"[init] Service '{name}' stopped gracefully."}