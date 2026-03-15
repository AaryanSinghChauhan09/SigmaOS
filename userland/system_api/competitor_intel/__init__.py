"""
competitor_intel.py — backward-compat shim.
Real implementation lives in competitor_intel/ package.
"""

from .competitor_intel.SigmaCompetitorIntelligence import *  # noqa

__all__ = ['SigmaCompetitorIntelligence']

"""Auto-generated package __init__.py"""
from .sigmacompetitorintelligence import *  # noqa: F401, F403
