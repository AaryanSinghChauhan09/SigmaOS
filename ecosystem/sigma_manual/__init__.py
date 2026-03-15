"""
sigma_manual.py — backward-compat shim.
Real implementation lives in sigma_manual/ package.
"""

from .sigma_manual.SigmaManual import *  # noqa

__all__ = ['SigmaManual']

"""Auto-generated package __init__.py"""
from .sigmamanual import *  # noqa: F401, F403
