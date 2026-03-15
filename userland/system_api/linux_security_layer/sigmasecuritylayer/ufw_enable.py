# Generated method: SigmaSecurityLayer.ufw_enable
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def ufw_enable(self):
        self.state['ufw']['status'] = 'active'
        self._save_state()
        return 'Firewall is active and enabled on system startup.'