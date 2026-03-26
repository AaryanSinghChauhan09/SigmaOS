"""
_show_control_center.py — backward-compat shim.
Real implementation lives in _show_control_center/ package.
"""

from ._show_control_center.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
