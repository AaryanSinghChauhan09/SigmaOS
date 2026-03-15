from ..system_factory import get_factory
from ..system.notification_bus import IObserver, get_system_bus

class AnalyticsDashboard(IObserver):
    """
    Observer-based Analytics Dashboard.
    Visualizes system events in real-time.
    """