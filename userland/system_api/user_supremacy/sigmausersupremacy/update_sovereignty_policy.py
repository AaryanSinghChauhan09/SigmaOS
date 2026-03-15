# Generated method: SigmaUserSupremacy.update_sovereignty_policy
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaUserSupremacy:
    def update_sovereignty_policy(self, policy='BLOCK'):
        self.forced_updates = policy == 'AUTO'
        return f"Update Policy: Set to '{policy}'. User is the sole authority."