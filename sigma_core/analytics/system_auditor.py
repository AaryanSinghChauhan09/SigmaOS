from ..interfaces.base_sovereign import SovereignModule
from ..interfaces.event_interfaces import IEventObserver

class SystemAuditor(SovereignModule, IEventObserver):
    """
    System Auditor Shard.
    Demonstrates Observer Pattern by listening to system events.
    """
    def __init__(self):
        super().__init__("SYSTEM_AUDITOR")
        self._logs = []

    def on_event(self, event_type, data):
        log_entry = f"AUDIT: [{event_type}] - {data}"
        print(f"[AUDITOR] {log_entry}")
        self._logs.append(log_entry)

    def execute(self, action=None):
        if action == "GET_LOGS":
            return self._logs
        return f"AUDITOR_STATUS_{self.status}"

    def initialize(self):
        print("[AUDITOR] Auditor Shard Active.")

    def shutdown(self):
        pass

    def health_check(self):
        return True
