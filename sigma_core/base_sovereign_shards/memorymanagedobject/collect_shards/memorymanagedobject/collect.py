# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class MemoryManagedObject:
    def collect(self):
        """Trigger explicit garbage collection."""
        gc.collect()