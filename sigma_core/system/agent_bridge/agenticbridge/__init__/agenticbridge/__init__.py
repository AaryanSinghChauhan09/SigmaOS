# Generated method: AgenticBridge.__init__
import json
import os
import time
from typing import Dict, Any

class AgenticBridge:
    def __init__(self, kernel, bridge_path: str='sigma_storage/agent_bridge'):
        self.kernel = kernel
        self.bridge_path = bridge_path
        os.makedirs(self.bridge_path, exist_ok=True)
        self.inbox = os.path.join(self.bridge_path, 'inbound.json')
        self.outbox = os.path.join(self.bridge_path, 'outbound.json')