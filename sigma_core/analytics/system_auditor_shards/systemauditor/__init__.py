from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventObserver

from ._base import SystemAuditor

class SystemAuditor:
    def __init__(self):
        super().__init__('SYSTEM_AUDITOR')
        self._logs = []