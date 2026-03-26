from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    def initialize(self):
        print(f"[GUI] Page '{self.name}' Initializing...")
        self.build_ui()