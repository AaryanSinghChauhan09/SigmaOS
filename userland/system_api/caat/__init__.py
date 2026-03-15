"""
caat.py — backward-compat shim.
Real implementation lives in caat/ package.
"""

from .caat.SigmaCAAT import *  # noqa

__all__ = ['SigmaCAAT']

"""Auto-generated package __init__.py"""
from .sigmacaat import *  # noqa: F401, F403
