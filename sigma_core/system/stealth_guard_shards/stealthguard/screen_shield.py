# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
from .base_sovereign import SigmaModule

class StealthGuard:
    def screen_shield(self):
        """Prevents unauthorized UI captures."""
        if self.active:
            return 'UI_MASKED'
        return 'UI_VISIBLE'