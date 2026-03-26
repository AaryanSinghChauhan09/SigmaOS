from functools import lru_cache
from abc import ABC, abstractmethod

from ._base import IDataShard

class IDataShard:
    @abstractmethod
    def get_data(self) -> bytes:
        raise NotImplementedError