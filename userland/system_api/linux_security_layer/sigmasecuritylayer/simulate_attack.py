# Generated method: SigmaSecurityLayer.simulate_attack
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def simulate_attack(self, ip: str, jail: str='ssh'):
        """Simulates a brute force attack to trigger fail2ban."""
        msg = ''
        if jail in self.state['fail2ban']['jails']:
            j = self.state['fail2ban']['jails'][jail]
            if ip not in j['banned_ips']:
                j['banned_ips'].append(ip)
                msg = f"[fail2ban] IP {ip} has been BANNED on jail '{jail}'."
            else:
                msg = f'[fail2ban] IP {ip} is already banned.'
            self._save_state()
        return msg