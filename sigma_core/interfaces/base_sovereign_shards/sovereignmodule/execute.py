from abc import ABC, abstractmethod
import time

from ._base import SovereignModule

class SovereignModule:
    @abstractmethod
    def execute(self, *args, **kwargs):
        pass