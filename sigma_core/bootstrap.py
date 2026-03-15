"""
SigmaOS Apex Optimized Shim (v4.4)
"""
from .bootstrap_shards.displaytextcommand._base import DisplayTextCommand
def bootstrap_zenith(*args, **kwargs):
    import importlib
    mod = importlib.import_module('sigma_core.bootstrap_shards.bootstrap_zenith')
    return getattr(mod, 'bootstrap_zenith')(*args, **kwargs)
