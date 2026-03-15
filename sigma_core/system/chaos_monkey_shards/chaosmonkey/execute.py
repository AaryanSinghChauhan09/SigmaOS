from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.resilience_interfaces import IChaosResilience
import random

from ._base import ChaosMonkey

class ChaosMonkey:
    def execute(self, action=None):
        if action == 'TICK':
            return self.perturb_system()
        return f'PERTURBATIONS_{self._perturbation_count}'