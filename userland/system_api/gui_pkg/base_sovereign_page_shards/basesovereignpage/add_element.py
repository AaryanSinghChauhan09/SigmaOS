from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    def add_element(self, element):
        """Encapsulation: Indirect access to page elements."""
        self._elements.append(element)