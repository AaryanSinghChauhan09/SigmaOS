from functools import lru_cache
from abc import ABC, abstractmethod

from ._base import ISafetyInvariant

class ISafetyInvariant:
    @abstractmethod
    @lru_cache(128)
    def verify(self, shard_logic: str) -> bool:
        raise NotImplementedError