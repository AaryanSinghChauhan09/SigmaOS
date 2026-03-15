"""
Auto-split from userland\system_api\adaptive_kernel.py — SigmaAdaptiveKernel.get_current_params
"""

import time
import threading
from enum import Enum, auto



class SigmaAdaptiveKernel:
    def get_current_params(self) -> dict:
        """Returns the currently active kernel parameter set."""
        return _PROFILE_PARAMS[self.current_profile]
