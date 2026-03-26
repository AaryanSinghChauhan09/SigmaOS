"""
_bind_shortcuts.py — backward-compat shim.
Real implementation lives in _bind_shortcuts/ package.
"""

from ._bind_shortcuts.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
