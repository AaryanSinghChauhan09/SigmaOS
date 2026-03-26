from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import LoggingDecorator

class LoggingDecorator:
    def execute(self, action, *args, **kwargs):
        name = self.metadata.get('name', 'UNKNOWN')
        print(f'[LOG] {name} Shard Execution Start: {action}')
        res = super().execute(action, *args, **kwargs)
        print(f'[LOG] {name} Shard Execution Complete')
        return res