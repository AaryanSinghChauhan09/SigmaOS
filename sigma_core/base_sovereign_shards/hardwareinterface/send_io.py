# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class HardwareInterface:
    @abstractmethod
    def send_io(self, port, data):
        pass