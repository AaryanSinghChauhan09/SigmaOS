"""
SigmaOS Apex Optimized Shim (v4.8.2)
"""
def get_device_manager(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.system.device_manager_shards.get_device_manager')
    return getattr(mod, 'get_device_manager')(*args, **kwargs)
