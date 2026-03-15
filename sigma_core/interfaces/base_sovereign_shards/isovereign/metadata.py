from abc import ABC, abstractmethod
import time

from ._base import ISovereign

class ISovereign:
    @property
    @abstractmethod
    def metadata(self) -> dict:
        pass