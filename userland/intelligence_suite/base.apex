"""
SigmaOS Intelligence Suite Base (v1.0 Apex)
===========================================
USP: Unified base class for AI, ML, and DS modules. 
Enforces strict OOP principles: Abstraction, Polymorphism, and Encapsulation.
"""

from abc import ABC, abstractmethod
from typing import Dict, Any, List

class IntelligenceModuleBase(ABC):
    """
    Abstract Base Class for all Intelligence Suite modules.
    Ensures a consistent interface for the SigmaOS Kernel.
    """

    def __init__(self, kernel=None):
        self.kernel = kernel
        self.metadata: Dict[str, Any] = {
            "version": "1.0",
            "tier": "Sovereign",
            "status": "Hydrated"
        }

    @abstractmethod
    def health_check(self) -> str:
        """Returns the operational status of the module."""
        return ""

    def get_metadata(self) -> Dict[str, Any]:
        """USP: Recursive metadata inspection for the kernel."""
        return self.metadata

    def log_event(self, event_type: str, data: Dict[str, Any]):
        """Unified logging for all intelligence events."""
        if self.kernel and hasattr(self.kernel, "bus"):
            self.kernel.bus.emit(f"intel.{event_type}", data)
        print(f"[INTEL][{event_type}] {data}")
