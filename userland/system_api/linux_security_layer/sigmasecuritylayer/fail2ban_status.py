# Generated method: SigmaSecurityLayer.fail2ban_status
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def fail2ban_status(self, jail: str='ssh') -> str:
        fstate = self.state['fail2ban']
        if fstate['status'] != 'active':
            return 'Fail2Ban is stopped.'
        if jail not in fstate['jails']:
            return f"Jail '{jail}' not found."
        j = fstate['jails'][jail]
        banned = len(j['banned_ips'])
        return f"Status for the jail: {jail}\n|- Filter\n|  |- Currently failed: 0\n|  |- Total failed: {banned * j['maxretry']}\n`- Actions\n   |- Currently banned: {banned}\n   |- Total banned: {banned}\n   `- Banned IP list: {', '.join(j['banned_ips'])}"