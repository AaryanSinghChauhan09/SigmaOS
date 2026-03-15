# Generated method: SigmaNetworkGuardian.health_check
import time
import threading
from typing import Dict, List, Any

class SigmaNetworkGuardian:
    def health_check(self) -> str:
        s = self.get_stats()
        return f"OK — NetworkGuardian | QOS: {s['qos_policy']} | Conns: {s['active_connections']} | Trackers Crushed: {s['telemetry_blocked']}"