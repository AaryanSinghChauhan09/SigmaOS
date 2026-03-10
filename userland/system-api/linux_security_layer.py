"""
SigmaOS Professional Security Layer
===================================
Focuses on enterprise-grade Linux security:
- ufw (Uncomplicated Firewall) parity
- SELinux (Security-Enhanced Linux) Mandatory Access Control
- fail2ban (Intrusion prevention / brute force mitigation)
"""

import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def __init__(self, kernel):
        self.kernel = kernel
        self.config_dir = Path(r'C:\Users\Sovereign-User\.gemini\antigravity\scratch\SigmaOS\config\security_layer')
        self.config_dir.mkdir(parents=True, exist_ok=True)
        
        self.state_file = self.config_dir / 'security_state.json'
        self.state = self._load_state()

    def _load_state(self) -> Dict[str, Any]:
        default = {
            "ufw": {"status": "inactive", "default_incoming": "deny", "default_outgoing": "allow", "rules": []},
            "selinux": {"mode": "permissive"}, # permissive, enforcing, disabled
            "fail2ban": {"status": "active", "jails": {"ssh": {"maxretry": 3, "bantime": 3600, "banned_ips": []}}}
        }
        if self.state_file.exists():
            try:
                with open(self.state_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except: pass
        return default

    def _save_state(self):
        with open(self.state_file, 'w', encoding='utf-8') as f:
            json.dump(self.state, f, indent=4)

    # UFW Commands
    def ufw_enable(self):
        self.state["ufw"]["status"] = "active"
        self._save_state()
        return "Firewall is active and enabled on system startup."

    def ufw_disable(self):
        self.state["ufw"]["status"] = "inactive"
        self._save_state()
        return "Firewall stopped and disabled on system startup."

    def ufw_allow(self, port: str):
        rule = {"action": "allow", "port": port}
        self.state["ufw"]["rules"].append(rule)
        self._save_state()
        return f"Rule added: ALLOW {port}"

    # SELinux Commands
    def selinux_setenforce(self, mode: int):
        status = "enforcing" if mode == 1 else "permissive"
        self.state["selinux"]["mode"] = status
        self._save_state()
        return f"SELinux mode is now {status}."

    def selinux_getenforce(self) -> str:
        return self.state["selinux"]["mode"].capitalize()

    # Fail2Ban Commands
    def fail2ban_status(self, jail: str = "ssh") -> str:
        fstate = self.state["fail2ban"]
        if fstate["status"] != "active":
            return "Fail2Ban is stopped."
        if jail not in fstate["jails"]:
            return f"Jail '{jail}' not found."
        
        j = fstate["jails"][jail]
        banned = len(j["banned_ips"])
        return f"Status for the jail: {jail}\n|- Filter\n|  |- Currently failed: 0\n|  |- Total failed: {banned * j['maxretry']}\n`- Actions\n   |- Currently banned: {banned}\n   |- Total banned: {banned}\n   `- Banned IP list: {', '.join(j['banned_ips'])}"
        
    def simulate_attack(self, ip: str, jail: str = "ssh"):
        """Simulates a brute force attack to trigger fail2ban."""
        msg = ""
        if jail in self.state["fail2ban"]["jails"]:
            j = self.state["fail2ban"]["jails"][jail]
            if ip not in j["banned_ips"]:
                j["banned_ips"].append(ip)
                msg = f"[fail2ban] IP {ip} has been BANNED on jail '{jail}'."
            else:
                msg = f"[fail2ban] IP {ip} is already banned."
            self._save_state()
        return msg

    def health_check(self) -> str:
        ufw_st = self.state['ufw']['status']
        sel_st = self.state['selinux']['mode']
        fb_st = self.state['fail2ban']['status']
        return f"OK — SecurityLayer: UFW={ufw_st.upper()} | SELinux={sel_st.upper()} | Fail2Ban={fb_st.upper()}"
