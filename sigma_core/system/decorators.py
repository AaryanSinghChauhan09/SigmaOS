from ..interfaces.base_sovereign import ISovereign
import time
import traceback

class ShardDecorator(ISovereign):
    """
    Base Decorator / Proxy for Sovereign Components.
    Proxies all attributes to the underlying component.
    """
    def __init__(self, component: ISovereign):
        self._component = component

    def execute(self, *args, **kwargs):
        return self._component.execute(*args, **kwargs)

    @property
    def metadata(self) -> dict:
        return self._component.metadata

    def __getattr__(self, name):
        """
        Proxy Pattern: Forward unknown attribute requests to the wrapped component.
        """
        return getattr(self._component, name)

class LoggingDecorator(ShardDecorator):
    """
    Logging Shard Wrapper.
    """
    def execute(self, *args, **kwargs):
        name = self.metadata.get('name', 'UNKNOWN')
        print(f"[LOG] {name} Shard Execution Start")
        res = super().execute(*args, **kwargs)
        print(f"[LOG] {name} Shard Execution Complete")
        return res

class ResilienceDecorator(ShardDecorator):
    """
    Fault Tolerance Shard Wrapper.
    """
    def execute(self, *args, **kwargs):
        try:
            return super().execute(*args, **kwargs)
        except Exception as e:
            print(f"[ZENITH-FAULT] Exception in {self.metadata.get('name')}: {e}")
            return {"error": "AUTO_REMEDY_ACTIVE"}
