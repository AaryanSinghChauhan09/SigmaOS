"""
sovereign_sync.py — backward-compat shim.
Real implementation lives in sovereign_sync/ package.
"""

from .sovereign_sync.SigmaSovereignSync import *  # noqa

__all__ = ['SigmaSovereignSync']

"""Auto-generated package __init__.py"""
from .sigmasovereignsync import *  # noqa: F401, F403
