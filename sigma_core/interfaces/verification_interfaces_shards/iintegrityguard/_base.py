from functools import lru_cache
from abc import ABC, abstractmethod


class IIntegrityGuard(ABC):
    """
    Enforces safety proofs on the system.
    """