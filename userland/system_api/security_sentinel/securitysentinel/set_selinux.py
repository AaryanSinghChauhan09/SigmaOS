# Generated method: SecuritySentinel.set_selinux
import time
import random
from typing import List, Dict

class SecuritySentinel:
    def set_selinux(self, mode: str) -> str:
        if mode in ['Enforcing', 'Permissive', 'Disabled']:
            self.selinux_mode = mode
            self._log(f'MAC Layer (SELinux): Context set to {mode}.')
            return f'Security Context: {mode}'
        return 'Invalid Mode'