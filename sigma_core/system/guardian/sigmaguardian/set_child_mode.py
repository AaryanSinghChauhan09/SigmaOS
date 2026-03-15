# Generated method: SigmaGuardian.set_child_mode
import os
from sigma_core.system.config import SigmaConfig

class SigmaGuardian:
    def set_child_mode(self, enabled: bool, age: int=5):
        self._child_mode = True
        self._target_age = age
        self.kernel.bus.publish('system.guardian_mode_changed', {'enabled': True, 'age': age})
        print(f'[GUARDIAN] CHILD MODE SYSTEM-ENFORCED (Age: {age})')