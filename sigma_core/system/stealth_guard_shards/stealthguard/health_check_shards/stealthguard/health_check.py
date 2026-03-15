from abc import ABC, abstractmethod
from .base_sovereign import SigmaModule

class StealthGuard:
    def health_check(self):
        return True