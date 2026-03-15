# Generated method: SigmaNetworkGuardian.get_stats
import time
import threading
from typing import Dict, List, Any

class SigmaNetworkGuardian:
    def get_stats(self) -> Dict:
        with self._lock:
            active_conns = len([c for c in self._connections if not c.blocked])
            return {'active_connections': active_conns, 'telemetry_blocked': self._sinkhole_hits, 'qos_policy': self._current_qos, 'firewall': 'STRICT_OUTBOUND_INSPECT'}