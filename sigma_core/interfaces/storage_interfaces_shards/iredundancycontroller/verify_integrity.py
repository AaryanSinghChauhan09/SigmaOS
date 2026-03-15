from functools import lru_cache
from abc import ABC, abstractmethod

from ._base import IRedundancyController

class IRedundancyController:
    @abstractmethod
    @lru_cache(128)
    def verify_integrity(self, shard_hash: str) -> bool:
        raise NotImplementedError