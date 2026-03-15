"""
shell_game.py — backward-compat shim.
Real implementation lives in shell_game/ package.
"""

from .shell_game.WatchTheCup import *  # noqa

__all__ = ['WatchTheCup']

"""Auto-generated package __init__.py"""
from .watchthecup import *  # noqa: F401, F403
