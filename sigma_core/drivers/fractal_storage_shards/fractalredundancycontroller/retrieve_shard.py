from functools import lru_cache
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.storage_interfaces import IRedundancyController, IDataShard
import hashlib

from ._base import FractalRedundancyController

class FractalRedundancyController:
    def retrieve_shard(self, shard_hash: str) -> IDataShard:
        for nid, shards in self._nodes.items():
            if shard_hash in shards:
                print(f'[FRACTAL] Shard {str(shard_hash)[0:8]} retrieved from Node {nid}')
                return shards[shard_hash]
        raise FileNotFoundError(f'Shard {shard_hash} lost.')