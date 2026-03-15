"""
sentinel.py — backward-compat shim.
Real implementation lives in sentinel/ package.
"""

from .sentinel.SovereignSentinel import *  # noqa

__all__ = ['SovereignSentinel']

"""Auto-generated package __init__.py"""
from .sovereignsentinel import *  # noqa: F401, F403
