from sigma_core.interfaces.base_sovereign import ISovereign
from sigma_core.system.decorators import LoggingDecorator, ResilienceDecorator, MetricsDecorator, PrivacyDecorator
import threading

from ._base import SystemFactory

class SystemFactory:
    def get(self, name: str) -> ISovereign:
        """Retrieves an assembled Sovereign Unit."""
        if name not in self._registry:
            raise KeyError(f"Sovereign Unit '{name}' not found.")
        return self._registry[name]