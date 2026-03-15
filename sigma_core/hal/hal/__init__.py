"""
hal.py — backward-compat shim.
Real implementation lives in hal/ package.
"""

from .hal.SigmaHAL import *  # noqa

__all__ = ['SigmaHAL']

"""Auto-generated package __init__.py"""
from .sigmahal import *  # noqa: F401, F403
