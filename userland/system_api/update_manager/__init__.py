"""
update_manager.py — backward-compat shim.
Real implementation lives in update_manager/ package.
"""

from .update_manager._sha256 import *  # noqa
from .update_manager._DeltaShard import *  # noqa
from .update_manager.SigmaUpdateManager import *  # noqa

__all__ = ['_sha256', '_DeltaShard', 'SigmaUpdateManager']

"""Auto-generated package __init__.py"""
from ._sha256 import *  # noqa: F401, F403
from ._DeltaShard import *  # noqa: F401, F403
from .sigmaupdatemanager import *  # noqa: F401, F403
