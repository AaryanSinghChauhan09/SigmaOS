from functools import lru_cache
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.storage_interfaces import IRedundancyController, IDataShard
import hashlib

from ._base import FractalRedundancyController

class FractalRedundancyController:
    @lru_cache(128)
    def verify_integrity(self, shard_hash: str) -> bool:
        copies = sum((1 for nid, shards in self._nodes.items() if shard_hash in shards))
        print(f'[FRACTAL] {str(shard_hash)[0:8]} has {copies} healthy mirrors.')
        return copies > 0