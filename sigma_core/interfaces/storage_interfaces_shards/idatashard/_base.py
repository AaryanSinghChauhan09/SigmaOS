from functools import lru_cache
from abc import ABC, abstractmethod


class IDataShard(ABC):
    """
    Interface for a single unit of Fractal Storage.
    """