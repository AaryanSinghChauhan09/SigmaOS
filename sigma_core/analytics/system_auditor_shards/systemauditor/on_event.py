from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver

from ._base import SystemAuditor

class SystemAuditor:
    def on_event(self, event_type, data):
        log_entry = f'AUDIT: [{event_type}] - {data}'
        print(f'[AUDITOR] {log_entry}')
        self._logs.append(log_entry)