"""
unified_api.py — backward-compat shim.
Real implementation lives in unified_api/ package.
"""

from .unified_api.SigmaUnifiedAPI import *  # noqa

__all__ = ['SigmaUnifiedAPI']

"""Auto-generated package __init__.py"""
from .sigmaunifiedapi import *  # noqa: F401, F403
