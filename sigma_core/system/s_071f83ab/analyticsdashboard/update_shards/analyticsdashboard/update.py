from ..system_factory import get_factory
from ..system.notification_bus import IObserver, get_system_bus

class AnalyticsDashboard:
    def update(self, event_type, data):
        """Concrete implementation of IObserver."""
        self.stats['events'] += 1
        self.stats['last_event'] = event_type
        print(f'[DASHBOARD] Event Received: {event_type} | Data: {data}')