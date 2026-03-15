"""
update_manager.py — backward-compat shim.
Real implementation lives in update_manager/ package.
"""

from update_manager._sha256 import *  # noqa
from update_manager._DeltaShard import *  # noqa
from update_manager.SigmaUpdateManager import *  # noqa

__all__ = ['_sha256', '_DeltaShard', 'SigmaUpdateManager']
