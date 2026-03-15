# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class SigmaSovereignBase:
    def __init__(self, module_name):
        MemoryManagedObject.__init__(self)
        AccessProtected.__init__(self)
        self.module_name = module_name