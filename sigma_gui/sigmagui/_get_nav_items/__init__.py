"""
_get_nav_items.py — backward-compat shim.
Real implementation lives in _get_nav_items/ package.
"""

from ._get_nav_items.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
