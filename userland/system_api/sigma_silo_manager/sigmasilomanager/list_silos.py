# Generated method: SigmaSiloManager.list_silos
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaSiloManager:
    def list_silos(self) -> List[Dict]:
        return [{'id': s.id, 'name': s.name, 'os': s.os_type, 'status': s.status, 'ip': s.ip_address, 'resources': s.resources} for s in self.silos.values()]