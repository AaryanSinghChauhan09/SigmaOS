"""
accessibility.py — backward-compat shim.
Real implementation lives in accessibility/ package.
"""

from .accessibility.SigmaAccessibilityHub import *  # noqa

__all__ = ['SigmaAccessibilityHub']

"""Auto-generated package __init__.py"""
from .sigmaaccessibilityhub import *  # noqa: F401, F403
