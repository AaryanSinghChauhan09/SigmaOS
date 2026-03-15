"""
hal.py — backward-compat shim.
Real implementation lives in hal/ package.
"""

from hal.SigmaHAL import *  # noqa

__all__ = ['SigmaHAL']
