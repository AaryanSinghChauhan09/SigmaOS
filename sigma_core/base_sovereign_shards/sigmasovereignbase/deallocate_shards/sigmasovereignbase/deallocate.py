# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class SigmaSovereignBase:
    def deallocate(self):
        print(f'[MEM] Shard {self.module_name} deallocated.')
        self._allocated = False