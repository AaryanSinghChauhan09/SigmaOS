from ..system_factory import get_factory
from ..system.notification_bus import IObserver, get_system_bus

class AnalyticsDashboard:
    def get_summary(self):
        return self.stats