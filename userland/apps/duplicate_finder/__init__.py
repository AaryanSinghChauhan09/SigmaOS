"""
duplicate_finder.py — backward-compat shim.
Real implementation lives in duplicate_finder/ package.
"""

from .duplicate_finder.DuplicateFinder import *  # noqa

__all__ = ['DuplicateFinder']

"""Auto-generated package __init__.py"""
from .duplicatefinder import *  # noqa: F401, F403
