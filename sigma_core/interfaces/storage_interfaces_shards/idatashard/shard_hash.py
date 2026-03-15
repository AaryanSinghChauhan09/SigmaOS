from functools import lru_cache
from abc import ABC, abstractmethod

from ._base import IDataShard

class IDataShard:
    @property
    @abstractmethod
    def shard_hash(self) -> str:
        raise NotImplementedError