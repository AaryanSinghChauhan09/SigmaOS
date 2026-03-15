from abc import ABC, abstractmethod
import time

from ._base import SovereignModule

class SovereignModule:
    def __init__(self, name):
        super().__init__()
        self.name = name
        self.status = 'READY'