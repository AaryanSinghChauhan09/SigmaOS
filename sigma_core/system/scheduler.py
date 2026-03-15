"""
SigmaOS Apex Optimized Shim (v4.4)
"""
from .scheduler_shards.sovereignscheduler._base import SovereignScheduler
def get_scheduler(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.system.scheduler_shards.get_scheduler')
    return getattr(mod, 'get_scheduler')(*args, **kwargs)
