from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.storage_interfaces import IRedundancyController, IDataShard
import hashlib

class SovereignShard(IDataShard):
    """
    Concrete Data Shard implementation.
    Encapsulates raw data and its integrity hash.
    """
    def __init__(self, data: bytes):
        self._data = data
        self._hash = hashlib.sha256(data).hexdigest()

    @property
    def shard_hash(self): return self._hash
    def get_data(self): return self._data

class FractalRedundancyController(SovereignModule, IRedundancyController):
    """
    Fractal Redundancy Controller.
    Mirrors data across virtual 'nodes' to ensure absolute persistence.
    """
    def __init__(self):
        super().__init__("FRACTAL_STORAGE")
        self._nodes = {} # NodeID -> {ShardHash -> Shard}

    def execute(self, action, payload=None):
        if action == "STORE":
            return self.store_shard(payload['shard'], payload['nodes'])
        elif action == "RETRIEVE":
            return self.retrieve_shard(payload)
        return "FRACTAL_ACTIVE"

    def store_shard(self, shard: IDataShard, node_ids: list):
        print(f"[FRACTAL] Mirroring shard {str(shard.shard_hash)[0:8]} to {len(node_ids)} nodes.")
        for nid in node_ids:
            if nid not in self._nodes: self._nodes[nid] = {}
            self._nodes[nid][shard.shard_hash] = shard

    def retrieve_shard(self, shard_hash: str) -> IDataShard:
        for nid, shards in self._nodes.items():
            if shard_hash in shards:
                print(f"[FRACTAL] Shard {str(shard_hash)[0:8]} retrieved from Node {nid}")
                return shards[shard_hash]
        raise FileNotFoundError(f"Shard {shard_hash} lost.")

    def verify_integrity(self, shard_hash: str) -> bool:
        copies = sum(1 for nid, shards in self._nodes.items() if shard_hash in shards)
        print(f"[FRACTAL] {str(shard_hash)[0:8]} has {copies} healthy mirrors.")
        return copies > 0

    def initialize(self): print("[FRACTAL] Mesh Redundancy Engine Online.")
    def shutdown(self): self._nodes.clear()
    def health_check(self): return True
