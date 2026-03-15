from ..interfaces.base_sovereign import SigmaModule
import time

class ResourcePool:
    def __init__(self, resource_type, size=10):
        super().__init__(f'POOL_{resource_type.upper()}')
        self._available = [f'{resource_type}_{i}' for i in range(size)]
        self._in_use = {}