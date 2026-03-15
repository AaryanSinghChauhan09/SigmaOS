"""
adaptive_governor.py — backward-compat shim.
Real implementation lives in adaptive_governor/ package.
"""

from adaptive_governor.AdaptiveGovernor import *  # noqa

__all__ = ['AdaptiveGovernor']
