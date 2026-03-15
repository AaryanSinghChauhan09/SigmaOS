"""
space_explorer.py — backward-compat shim.
Real implementation lives in space_explorer/ package.
"""

from .space_explorer.SpaceExplorer import *  # noqa

__all__ = ['SpaceExplorer']

"""Auto-generated package __init__.py"""
from .spaceexplorer import *  # noqa: F401, F403
