"""
pulseplayer.py — backward-compat shim.
Real implementation lives in pulseplayer/ package.
"""

from .pulseplayer.PulsePlayer import *  # noqa

__all__ = ['PulsePlayer']

"""Auto-generated package __init__.py"""
from .pulseplayer import *  # noqa: F401, F403
