from abc import ABC, abstractmethod
import time

from ._base import ISovereign

class ISovereign:
    @abstractmethod
    def execute(self, *args, **kwargs):
        pass