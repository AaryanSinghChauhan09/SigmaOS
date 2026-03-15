# Generated method: SigmaSilo.__init__
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaSilo:
    def __init__(self, name: str, os_type: str, resources: Dict):
        self.id = str(uuid.uuid4())[:8]
        self.name = name
        self.os_type = os_type
        self.resources = resources
        self.status = 'CREATED'
        self.uptime = 0
        self.ip_address = f'10.0.silo.{random.randint(2, 254)}'
        self.start_time = 0