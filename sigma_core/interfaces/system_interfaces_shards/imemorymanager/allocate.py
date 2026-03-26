from abc import ABC, abstractmethod

from ._base import IMemoryManager

class IMemoryManager:
    @abstractmethod
    def allocate(self, size):
        raise NotImplementedError