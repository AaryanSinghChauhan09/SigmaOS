"""
SigmaOS Core Interfaces & SOLID Contracts
==========================================
Enforces SRP, OCP, LSP, ISP, and DIP across the Sovereign ecosystem.
"""

from abc import ABC, abstractmethod
from typing import Dict, Any, Optional

class ISigmaModule(ABC):
    """
    Interface Segregation: Modules only implement what they need.
    Dependency Inversion: Kernel depends on this abstraction, not concrete modules.
    """
    @abstractmethod
    def get_module_id(self) -> str:
        pass

    @abstractmethod
    def health_check(self) -> str:
        pass

class ISigmaService(ISigmaModule):
    """Extended Interface for long-running background services."""
    @abstractmethod
    def start_service(self) -> None:
        pass

    @abstractmethod
    def stop_service(self) -> None:
        pass

class SigmaModuleBase(ISigmaModule):
    """
    Base class providing common functionality for Liskov Substitution Principle.
    All modules can be substituted for this base class in the Registry.
    """
    def __init__(self, kernel):
        self.kernel = kernel

    def get_module_id(self) -> str:
        return self.__class__.__name__.lower()

    def health_check(self) -> str:
        return "OK"

    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, "ledger"):
            self.kernel.ledger.commit(self.get_module_id(), action, context)
