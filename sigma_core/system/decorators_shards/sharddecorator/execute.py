from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import ShardDecorator

class ShardDecorator:
    def execute(self, action, *args, **kwargs):
        return self._component.execute(action, *args, **kwargs)