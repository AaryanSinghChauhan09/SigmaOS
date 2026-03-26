"""
sigma_ai_nexus.py — backward-compat shim.
Real implementation lives in sigma_ai_nexus/ package.
"""

from .sigma_ai_nexus.SigmaAINexus import *  # noqa

__all__ = ['SigmaAINexus']

"""Auto-generated package __init__.py"""
from .sigmaainexus import *  # noqa: F401, F403
