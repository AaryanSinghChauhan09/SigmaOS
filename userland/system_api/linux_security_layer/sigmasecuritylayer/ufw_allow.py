# Generated method: SigmaSecurityLayer.ufw_allow
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def ufw_allow(self, port: str):
        rule = {'action': 'allow', 'port': port}
        self.state['ufw']['rules'].append(rule)
        self._save_state()
        return f'Rule added: ALLOW {port}'