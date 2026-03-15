from abc import ABC, abstractmethod

class ISafetyInvariant(ABC):
    """
    Formal Verification Contract.
    Ensures mathematical certainty of shard execution.
    """
    @abstractmethod
    def verify(self, shard_logic: str) -> bool:
        raise NotImplementedError

class IIntegrityGuard(ABC):
    """
    Enforces safety proofs on the system.
    """
    @abstractmethod
    def validate_shard(self, shard_id, logic: str):
        raise NotImplementedError
