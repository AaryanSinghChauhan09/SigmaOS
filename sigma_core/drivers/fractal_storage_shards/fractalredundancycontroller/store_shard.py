from functools import lru_cache
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.storage_interfaces import IRedundancyController, IDataShard
import hashlib

from ._base import FractalRedundancyController

class FractalRedundancyController:
    def store_shard(self, shard: IDataShard, node_ids: list):
        print(f'[FRACTAL] Mirroring shard {str(shard.shard_hash)[0:8]} to {len(node_ids)} nodes.')
        for nid in node_ids:
            if nid not in self._nodes:
                self._nodes[nid] = {}
            self._nodes[nid][shard.shard_hash] = shard