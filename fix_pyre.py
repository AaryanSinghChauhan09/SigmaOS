"""
SigmaOS Apex Optimized Shim (v4.9)
"""
def fix_apex(*args, **kwargs):
    import importlib
    mod = importlib.import_module('.fix_pyre_shards.fix_apex')
    return getattr(mod, 'fix_apex')(*args, **kwargs)
