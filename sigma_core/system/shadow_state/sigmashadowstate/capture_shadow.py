# Generated method: SigmaShadowState.capture_shadow
import time
import copy
import threading
from typing import Dict, Any

class SigmaShadowState:
    def capture_shadow(self, module_key: str):
        """Creates a serialized snapshot of a module's essential state."""
        module = self.kernel.registry.get(module_key)
        if not module:
            return
        state = {}
        if hasattr(module, 'get_state') and callable(module.get_state):
            state = module.get_state()
        elif hasattr(module, '__dict__'):
            blacklist_types = ('Lock', 'RLock', 'Event', 'Thread', 'ModuleRegistry', 'EventBus', 'SigmaKernel')
            for k, v in module.__dict__.items():
                if k.startswith('__') or callable(v):
                    continue
                type_name = str(type(v))
                if any((bt in type_name for bt in blacklist_types)):
                    continue
                state[k] = v
        try:
            self._shadows[module_key] = copy.deepcopy(state)
            self._last_sync[module_key] = time.time()
            self.kernel.bus.emit('shadow.captured', {'module': module_key, 'ts': self._last_sync[module_key]})
        except Exception as e:
            self.kernel.bus.emit('shadow.capture_error', {'module': module_key, 'error': str(e)})