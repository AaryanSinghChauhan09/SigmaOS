from abc import ABC, abstractmethod

class IPrivacyContract(ABC):
    """
    Contract for Data Purpose-of-Use.
    Enforces Deterministic Privacy.
    """
    @property
    @abstractmethod
    def allowed_purpose(self) -> str:
        raise NotImplementedError

class IPrivacyGuard(ABC):
    """
    Interface for the Privacy Enforcement Layer.
    """
    @abstractmethod
    def authorize_access(self, data_tag: str, requester_purpose: str) -> bool:
        raise NotImplementedError

class IChaosResilience(ABC):
    """
    Interface for Chaos-based stress testing.
    """
    @abstractmethod
    def perturb_system(self):
        raise NotImplementedError
