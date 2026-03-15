from ..interfaces.base_sovereign import SigmaModule
import time

class ResourcePool:
    def acquire(self, client_id):
        if not self._available:
            print(f'[RESOURCES] Pool {self._name} Exhausted!')
            return None
        res = self._available.pop()
        self._in_use[client_id] = res
        return res