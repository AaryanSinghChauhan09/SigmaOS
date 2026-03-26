from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import ShardDecorator

class ShardDecorator:
    def __getattr__(self, name):
        """
            Proxy Pattern: Forward unknown attribute requests to the wrapped component.
            """
        return getattr(self._component, name)