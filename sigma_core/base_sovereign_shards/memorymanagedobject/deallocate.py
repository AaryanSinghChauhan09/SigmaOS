# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class MemoryManagedObject:
    @abstractmethod
    def deallocate(self):
        """Must be implemented to clear native buffers."""
        pass