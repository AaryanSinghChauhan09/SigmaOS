from functools import lru_cache
from abc import ABC, abstractmethod

from ._base import IIntegrityGuard

class IIntegrityGuard:
    @abstractmethod
    def validate_shard(self, shard_id, logic: str):
        raise NotImplementedError