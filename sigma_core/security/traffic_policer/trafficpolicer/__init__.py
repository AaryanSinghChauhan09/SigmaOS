# Generated method: TrafficPolicer.__init__
from typing import Set, Dict, Any

class TrafficPolicer:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.blocked_ips: Set[str] = set()
        self.blocked_shards: Set[str] = set()