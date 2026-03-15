"""
zero_trust.py — backward-compat shim.
Real implementation lives in zero_trust/ package.
"""

from zero_trust.SigmaZeroTrust import *  # noqa

__all__ = ['SigmaZeroTrust']
