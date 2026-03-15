from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.event_interfaces import IEventBus, IEventObserver
import threading
from ..sovereigneventbus._base import SovereignEventBus

def get_event_bus() -> SovereignEventBus:
    return SovereignEventBus()