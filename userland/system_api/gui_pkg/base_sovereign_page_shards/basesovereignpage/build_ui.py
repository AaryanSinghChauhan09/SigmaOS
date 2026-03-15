from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    @abstractmethod
    def build_ui(self):
        """Abstraction: UI construction must be handled by subclasses."""
        pass