# Generated method: SigmaGuardian.check_access
import os
from sigma_core.system.config import SigmaConfig

class SigmaGuardian:
    def check_access(self, rating: str) -> bool:
        """Returns True if the rating is allowed in the current mode."""
        if not self._child_mode:
            return True
        return rating in self.SAFE_RATINGS