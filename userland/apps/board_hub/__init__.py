"""
board_hub.py — backward-compat shim.
Real implementation lives in board_hub/ package.
"""

from .board_hub.SovereignArcade import *  # noqa

__all__ = ['SovereignArcade']

"""Auto-generated package __init__.py"""
from .sovereignarcade import *  # noqa: F401, F403
