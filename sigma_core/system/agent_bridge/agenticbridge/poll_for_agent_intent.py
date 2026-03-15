# Generated method: AgenticBridge.poll_for_agent_intent
import json
import os
import time
from typing import Dict, Any

class AgenticBridge:
    def poll_for_agent_intent(self):
        """Checks if an agent has dropped a JSON intent in the bridge inbox."""
        if os.path.exists(self.inbox):
            try:
                with open(self.inbox, 'r') as f:
                    cmd = json.load(f)
                os.remove(self.inbox)
                self._dispatch_command(cmd)
            except Exception as e:
                print(f'[AGENT-BRIDGE] Malformed intent: {e}')