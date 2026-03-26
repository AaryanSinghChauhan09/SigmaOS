from functools import lru_cache
from abc import ABC, abstractmethod
from ..idatashard._base import IDataShard
from ._base import IRedundancyController

class IRedundancyController:
    @abstractmethod
    def store_shard(self, shard: IDataShard, node_ids: list):
        raise NotImplementedError