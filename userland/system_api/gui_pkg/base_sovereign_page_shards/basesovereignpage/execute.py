from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    def execute(self, action, *args, **kwargs):
        if action == 'RENDER':
            return self.build_ui()
        return super().execute(action, *args, **kwargs)