from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver

from ._base import SystemAuditor

class SystemAuditor:
    def execute(self, action=None):
        if action == 'GET_LOGS':
            return self._logs
        return f'AUDITOR_STATUS_{self.status}'