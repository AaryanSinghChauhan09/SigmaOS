# Generated method: AgenticBridge.push_telemetry
import json
import os
import time
from typing import Dict, Any

class AgenticBridge:
    def push_telemetry(self, data: Dict[str, Any]):
        """Proactively pushes OS state to the agent."""
        state_file = os.path.join(self.bridge_path, 'os_state.json')
        with open(state_file, 'w') as f:
            json.dump(data, f)