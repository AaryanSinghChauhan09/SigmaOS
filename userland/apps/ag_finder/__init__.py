"""
ag_finder.py — backward-compat shim.
Real implementation lives in ag_finder/ package.
"""

from .ag_finder.ToolsFinder import *  # noqa

__all__ = ['ToolsFinder']

"""Auto-generated package __init__.py"""
from .toolsfinder import *  # noqa: F401, F403
