# Generated method: SigmaGuardian.__init__
import os
from sigma_core.system.config import SigmaConfig

class SigmaGuardian:
    def __init__(self, kernel):
        self.kernel = kernel
        self.cfg = SigmaConfig()
        self._child_mode = True
        self._target_age = 5
        self.SAFE_RATINGS = ['G', 'U', 'All Ages', '0+']