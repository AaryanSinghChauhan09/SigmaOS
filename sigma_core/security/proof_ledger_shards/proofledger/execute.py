from functools import lru_cache
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.verification_interfaces import IIntegrityGuard, ISafetyInvariant
import hashlib

from ._base import ProofLedger

class ProofLedger:
    def execute(self, action, *args, **kwargs):
        if action == 'COUNT':
            return len(self._verified_hashes)
        return None