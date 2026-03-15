from abc import ABC, abstractmethod

class IMemoryManager:
    @abstractmethod
    def allocate(self, size):
        pass