"""
adaptive_kernel.py — backward-compat shim.
Real implementation lives in adaptive_kernel/ package.
"""

from .adaptive_kernel.SigmaAdaptiveKernel import *  # noqa

__all__ = ['SigmaAdaptiveKernel']

"""Auto-generated package __init__.py"""
from .sigmaadaptivekernel import *  # noqa: F401, F403
