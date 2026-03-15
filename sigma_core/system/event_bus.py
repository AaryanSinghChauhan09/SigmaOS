"""
SigmaOS Apex Optimized Shim (v4.4)
"""
from .event_bus_shards.sovereigneventbus._base import SovereignEventBus
def get_event_bus(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.system.event_bus_shards.get_event_bus')
    return getattr(mod, 'get_event_bus')(*args, **kwargs)
