from abc import ABC, abstractmethod
from .base_sovereign import SigmaModule

class StealthGuard:
    def deactivate(self):
        self.active = False
        print('[STEALTH] OS returning to Standard State.')
        return True