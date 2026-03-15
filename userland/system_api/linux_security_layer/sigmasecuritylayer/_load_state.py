# Generated method: SigmaSecurityLayer._load_state
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def _load_state(self) -> Dict[str, Any]:
        default = {'ufw': {'status': 'inactive', 'default_incoming': 'deny', 'default_outgoing': 'allow', 'rules': []}, 'selinux': {'mode': 'permissive'}, 'fail2ban': {'status': 'active', 'jails': {'ssh': {'maxretry': 3, 'bantime': 3600, 'banned_ips': []}}}}
        if self.state_file.exists():
            try:
                with open(self.state_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except:
                pass
        return default