from abc import ABC, abstractmethod
import importlib
from .base_sovereign import SigmaModule

class SelfHealer:
    def health_check(self):
        return True