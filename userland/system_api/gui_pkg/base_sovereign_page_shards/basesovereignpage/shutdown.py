from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    def shutdown(self):
        self._elements.clear()
        print(f"[GUI] Page '{self.name}' Shutdown.")