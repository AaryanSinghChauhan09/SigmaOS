from ..interfaces.base_sovereign import ISovereign
import time
import traceback

class ShardDecorator(ISovereign):
    def __init__(self, component: ISovereign):
        self._component = component
    def execute(self, *args, **kwargs):
        return self._component.execute(*args, **kwargs)
    @property
    def metadata(self) -> dict:
        return self._component.metadata

class LoggingDecorator(ShardDecorator):
    def execute(self, *args, **kwargs):
        print(f"[LOG] {self.metadata.get('name')} Start")
        res = super().execute(*args, **kwargs)
        print(f"[LOG] {self.metadata.get('name')} End")
        return res

class ResilienceDecorator(ShardDecorator):
    def execute(self, *args, **kwargs):
        try: return super().execute(*args, **kwargs)
        except: return {"error": "FAILED"}
