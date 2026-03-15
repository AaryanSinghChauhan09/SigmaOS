"""
conversion_engine.py — backward-compat shim.
Real implementation lives in conversion_engine/ package.
"""

from .conversion_engine.SigmaConversionEngine import *  # noqa

__all__ = ['SigmaConversionEngine']

"""Auto-generated package __init__.py"""
from .sigmaconversionengine import *  # noqa: F401, F403
