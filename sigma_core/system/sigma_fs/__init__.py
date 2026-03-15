"""
sigma_fs.py — backward-compat shim.
Real implementation lives in sigma_fs/ package.
"""

from .sigma_fs.SigmaFS import *  # noqa

__all__ = ['SigmaFS']

"""Auto-generated package __init__.py"""
from .sigmafs import *  # noqa: F401, F403
