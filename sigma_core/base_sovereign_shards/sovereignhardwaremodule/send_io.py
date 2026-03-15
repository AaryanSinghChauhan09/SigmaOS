# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import gc

class SovereignHardwareModule:
    def send_io(self, port, data):
        print(f'[HAL] Writing {data} to port {port}')