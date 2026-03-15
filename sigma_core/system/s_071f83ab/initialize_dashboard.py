# SigmaOS Shard: initialize_dashboard
from ..system_factory import get_factory
from ..system.notification_bus import IObserver, get_system_bus

def initialize_dashboard():
    dashboard = AnalyticsDashboard()
    get_factory().register_component('Dashboard', dashboard)
    return dashboard