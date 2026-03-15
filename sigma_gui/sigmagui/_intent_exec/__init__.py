"""
_intent_exec.py — backward-compat shim.
Real implementation lives in _intent_exec/ package.
"""

from ._intent_exec.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
