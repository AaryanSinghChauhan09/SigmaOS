from functools import lru_cache
from abc import ABC, abstractmethod
from ..idatashard._base import IDataShard
from ._base import IRedundancyController

class IRedundancyController:
    @abstractmethod
    def retrieve_shard(self, shard_hash: str) -> IDataShard:
        raise NotImplementedError