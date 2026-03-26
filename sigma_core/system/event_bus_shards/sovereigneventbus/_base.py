from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading


class SovereignEventBus(SovereignModule, IEventBus):
    """
    Sovereign Event Bus (Singleton).
    Mediates communication between decoupled shards.
    """
    _instance = None
    _lock = threading.Lock()