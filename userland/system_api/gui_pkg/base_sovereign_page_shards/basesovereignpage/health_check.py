from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    def health_check(self):
        return self._status == 'READY'