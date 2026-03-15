from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

class ShardDecorator(ISovereign):
    """
    Base Decorator / Proxy for Sovereign Components.
    Proxies all attributes to the underlying component.
    """
    def __init__(self, component: ISovereign):
        self._component = component

    def execute(self, action, *args, **kwargs):
        return self._component.execute(action, *args, **kwargs)

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
    def execute(self, action, *args, **kwargs):
        name = self.metadata.get('name', 'UNKNOWN')
        print(f"[LOG] {name} Shard Execution Start: {action}")
        res = super().execute(action, *args, **kwargs)
        print(f"[LOG] {name} Shard Execution Complete")
        return res

class ResilienceDecorator(ShardDecorator):
    """
    Fault Tolerance Shard Wrapper.
    """
    def execute(self, action, *args, **kwargs):
        try:
            return super().execute(action, *args, **kwargs)
        except Exception as e:
            print(f"[ZENITH-FAULT] Exception in {self.metadata.get('name')}: {e}")
            return {"error": "AUTO_REMEDY_ACTIVE"}

class MetricsDecorator(ShardDecorator):
    """
    Analytics & Performance Collector.
    Tracks execution counts and latencies.
    """
    def __init__(self, component):
        super().__init__(component)
        self._exec_count = 0
        self._total_latency = 0.0

    def execute(self, action, *args, **kwargs):
        start = time.time()
        res = super().execute(action, *args, **kwargs)
        latency = time.time() - start
        
        self._exec_count += 1
        self._total_latency += latency
        
        print(f"[METRICS] {self.metadata.get('name')} | Total Execs: {self._exec_count} | Latency: {latency:.6f}s")
        return res

class PrivacyDecorator(ShardDecorator):
    """
    Zero-Trust Privacy Proxy.
    Ensures 'Purpose-of-Use' is authorized before execution.
    """
    def __init__(self, component, privacy_guard, required_tag):
        super().__init__(component)
        self._privacy_guard = privacy_guard
        self._required_tag = required_tag

    def execute(self, action, *args, **kwargs):
        """
        Overridden execute to check privacy contract.
        """
        if not self._privacy_guard:
            return super().execute(action, *args, **kwargs)

        # Context-aware privacy check
        purpose = kwargs.get('purpose')
        if self._privacy_guard.authorize_access(self._required_tag, purpose):
            return super().execute(action, *args, **kwargs)
        
        print(f"[PRIVACY-VETO] Blocked execution of {self.metadata.get('name')} - Purpose '{purpose}' unauthorized for tag '{self._required_tag}'")
        return {"error": "PRIVACY_VIOLATION", "required_tag": self._required_tag}
