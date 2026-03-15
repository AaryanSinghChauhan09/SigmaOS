"""
SigmaOS Apex Optimized Shim (v4.4)
"""
from .system_factory_shards.systemfactory._base import SystemFactory
def get_factory(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.system_factory_shards.get_factory')
    return getattr(mod, 'get_factory')(*args, **kwargs)
