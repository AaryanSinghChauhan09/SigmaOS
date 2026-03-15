"""
competitor_bridge.py — backward-compat shim.
Real implementation lives in competitor_bridge/ package.
"""

from .competitor_bridge.SigmaCompetitorBridge import *  # noqa

__all__ = ['SigmaCompetitorBridge']

"""Auto-generated package __init__.py"""
from .sigmacompetitorbridge import *  # noqa: F401, F403
