"""
SigmaOS Apex Optimized Shim (v4.4)
"""
from .sigma_refactor_v12_shards.constants import ROOT
from .sigma_refactor_v12_shards.constants import SKIP_DIRS
from .sigma_refactor_v12_shards.constants import PROTECTED_FILES
from .sigma_refactor_v12_shards.constants import PROTECTED_DIRS
from .sigma_refactor_v12_shards.constants import PERSONAL
from .sigma_refactor_v12_shards.constants import RELIGIOUS
from .sigma_refactor_v12_shards.constants import VULGAR
def sanitize(*args, **kwargs):
    import importlib
    mod = importlib.import_module('.sigma_refactor_v12_shards.sanitize')
    return getattr(mod, 'sanitize')(*args, **kwargs)
def process_file(*args, **kwargs):
    import importlib
    mod = importlib.import_module('.sigma_refactor_v12_shards.process_file')
    return getattr(mod, 'process_file')(*args, **kwargs)
