"""
zero_trust.py — backward-compat shim.
Real implementation lives in zero_trust/ package.
"""

from .zero_trust.SigmaZeroTrust import *  # noqa

__all__ = ['SigmaZeroTrust']

"""Auto-generated package __init__.py"""
from .sigmazerotrust import *  # noqa: F401, F403
