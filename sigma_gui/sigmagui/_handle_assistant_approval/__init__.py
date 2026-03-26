"""
_handle_assistant_approval.py — backward-compat shim.
Real implementation lives in _handle_assistant_approval/ package.
"""

from ._handle_assistant_approval.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
