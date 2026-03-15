# Generated method: TrafficPolicer.block_shard
from typing import Set, Dict, Any

class TrafficPolicer:
    def block_shard(self, shard_id: str):
        self.blocked_shards.add(shard_id)