"""
Auto-split from userland\system_api\adaptive_kernel.py — SigmaAdaptiveKernel.apply_profile
"""

import time
import threading
from enum import Enum, auto



class SigmaAdaptiveKernel:
    def apply_profile(self, profile: WorkloadProfile) -> dict:
        """
            Live hot-switch: applies new kernel params without reboot.
            Returns the applied parameter map.
            """
        params = _PROFILE_PARAMS[profile]
        old_profile = self.current_profile
        self.current_profile = profile
        self._transition_count += 1
        entry = {'timestamp': time.strftime('%Y-%m-%dT%H:%M:%S'), 'from': old_profile.name, 'to': profile.name, 'params': params, 'transition': self._transition_count}
        self._history.append(entry)
        return {'status': 'Applied', 'profile': profile.name, 'params': params, 'message': f'AdaptiveKernel: Hot-switched from {old_profile.name} → {profile.name} ({len(params)} parameters tuned).'}
