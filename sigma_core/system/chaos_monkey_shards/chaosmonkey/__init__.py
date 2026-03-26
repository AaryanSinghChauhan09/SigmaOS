from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IChaosResilience
import random

from ._base import ChaosMonkey

class ChaosMonkey:
    def __init__(self, target_factory):
        super().__init__('CHAOS_MONKEY')
        self._factory = target_factory
        self._perturbation_count = 0