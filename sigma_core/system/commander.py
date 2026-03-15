"""
SigmaOS Apex Optimized Shim (v4.8.2)
"""
def get_commander(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.system.commander_shards.get_commander')
    return getattr(mod, 'get_commander')(*args, **kwargs)
