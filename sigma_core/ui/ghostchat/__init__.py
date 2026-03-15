"""
ghostchat.py — backward-compat shim.
Real implementation lives in ghostchat/ package.
"""

from .ghostchat.SigmaGhostChat import *  # noqa

__all__ = ['SigmaGhostChat']

"""Auto-generated package __init__.py"""
from .sigmaghostchat import *  # noqa: F401, F403
