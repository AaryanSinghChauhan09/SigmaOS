"""
core.py — backward-compat shim.
Real implementation lives in core/ package.
"""

from .core.SigmaOSKernel import *  # noqa

__all__ = ['SigmaOSKernel']

"""Auto-generated package __init__.py"""
from .sigmaoskernel import *  # noqa: F401, F403
