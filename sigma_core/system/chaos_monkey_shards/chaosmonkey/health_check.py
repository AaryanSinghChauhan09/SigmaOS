from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IChaosResilience
import random

from ._base import ChaosMonkey

class ChaosMonkey:
    def health_check(self):
        return True