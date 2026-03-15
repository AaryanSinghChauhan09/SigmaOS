"""
pulseplayer.py — backward-compat shim.
Real implementation lives in pulseplayer/ package.
"""

from pulseplayer.PulsePlayer import *  # noqa

__all__ = ['PulsePlayer']
