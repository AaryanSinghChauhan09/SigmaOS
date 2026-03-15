# Generated method: SigmaShadowState.hot_swap
import time
import copy
import threading
from typing import Dict, Any

class SigmaShadowState:
    def hot_swap(self, module_key: str) -> bool:
        """
            USP: The 'Shadow Swap'. 
            Restores a module to its last known good state if the Watchdog trips it.
            """
        if module_key not in self._shadows:
            return False
        self._is_recovering = True
        module = self.kernel.registry.get(module_key)
        if not module:
            return False
        state = self._shadows[module_key]
        try:
            if hasattr(module, 'set_state'):
                module.set_state(state)
            else:
                for k, v in state.items():
                    setattr(module, k, copy.deepcopy(v))
            self.kernel.bus.emit('shadow.swapped', {'module': module_key, 'msg': 'Hot-Standby state RESTORED. System stabilized.'})
            self._is_recovering = False
            return True
        except Exception as e:
            self.kernel.bus.emit('shadow.swap_failed', {'module': module_key, 'error': str(e)})
            self._is_recovering = False
            return False