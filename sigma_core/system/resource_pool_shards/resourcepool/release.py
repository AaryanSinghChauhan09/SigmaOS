from ..interfaces.base_sovereign import SigmaModule
import time

class ResourcePool:
    def release(self, client_id):
        if client_id in self._in_use:
            res = self._in_use.pop(client_id)
            self._available.append(res)
            return True
        return False