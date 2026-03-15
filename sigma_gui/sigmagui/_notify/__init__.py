"""
_notify.py — backward-compat shim.
Real implementation lives in _notify/ package.
"""

from ._notify.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
