# Generated method: TrafficPolicer.status
from typing import Set, Dict, Any

class TrafficPolicer:
    def status(self) -> Dict[str, Any]:
        return {'rules': len(self.blocked_ips) + len(self.blocked_shards)}