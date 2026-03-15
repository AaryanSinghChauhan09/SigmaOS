# Generated method: SigmaSecurityLayer.selinux_setenforce
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def selinux_setenforce(self, mode: int):
        status = 'enforcing' if mode == 1 else 'permissive'
        self.state['selinux']['mode'] = status
        self._save_state()
        return f'SELinux mode is now {status}.'