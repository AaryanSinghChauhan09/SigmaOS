"""
writer.py — backward-compat shim.
Real implementation lives in writer/ package.
"""

from .writer.SovereignWriter import *  # noqa

__all__ = ['SovereignWriter']

"""Auto-generated package __init__.py"""
from .sovereignwriter import *  # noqa: F401, F403
