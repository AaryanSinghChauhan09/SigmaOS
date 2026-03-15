"""
sovereign_chat.py — backward-compat shim.
Real implementation lives in sovereign_chat/ package.
"""

from .sovereign_chat.SigmaSovereignMesh import *  # noqa

__all__ = ['SigmaSovereignMesh']

"""Auto-generated package __init__.py"""
from .sigmasovereignmesh import *  # noqa: F401, F403
