# Generated method: SecuritySentinel.health_check
import time
import random
from typing import List, Dict

class SecuritySentinel:
    def health_check(self) -> str:
        return f"OK — Sentinel: {self.selinux_mode} | Firewall: {('ON' if self.firewall_active else 'OFF')} | Jails: {self.stats['jail_count']}"