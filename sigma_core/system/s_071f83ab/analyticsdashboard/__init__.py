from ..system_factory import get_factory
from ..system.notification_bus import IObserver, get_system_bus

class AnalyticsDashboard:
    def __init__(self):
        self.stats = {'events': 0, 'last_event': None}
        get_system_bus().attach(self)