"""
_show_spotlight.py — backward-compat shim.
Real implementation lives in _show_spotlight/ package.
"""

from ._show_spotlight.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
