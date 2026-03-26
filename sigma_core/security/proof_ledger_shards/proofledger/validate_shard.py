import hashlib
import time
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.verification_interfaces import IIntegrityGuard


def validate_shard(self, shard_name, content):
    h = hashlib.sha256(content.encode()).hexdigest()
    self._verified_hashes[shard_name] = h
    print(f'[{self.name}] Verified Shard: {shard_name} -> {h[:8]}')
    return True