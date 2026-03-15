# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class SovereignModule:
    def boot(self, mode='STANDARD', priority=0):
        print(f'Booting {self.module_name} in {mode} mode (Priority: {priority})')