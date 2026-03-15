from abc import ABC, abstractmethod

class IDataShard(ABC):
    """
    Interface for a single unit of Fractal Storage.
    """
    @property
    @abstractmethod
    def shard_hash(self) -> str:
        raise NotImplementedError

    @abstractmethod
    def get_data(self) -> bytes:
        raise NotImplementedError

class IRedundancyController(ABC):
    """
    Interface for managing data mirroring and fractal distribution.
    Enforces Chaos Resilience (Fractal Redundancy).
    """
    @abstractmethod
    def store_shard(self, shard: IDataShard, node_ids: list):
        raise NotImplementedError

    @abstractmethod
    def retrieve_shard(self, shard_hash: str) -> IDataShard:
        raise NotImplementedError

    @abstractmethod
    def verify_integrity(self, shard_hash: str) -> bool:
        raise NotImplementedError
