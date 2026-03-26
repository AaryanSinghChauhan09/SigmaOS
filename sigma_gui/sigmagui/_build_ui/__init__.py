"""
_build_ui.py — backward-compat shim.
Real implementation lives in _build_ui/ package.
"""

from ._build_ui.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
