"""
aurapaint.py — backward-compat shim.
Real implementation lives in aurapaint/ package.
"""

from .aurapaint.AuraPaint import *  # noqa

__all__ = ['AuraPaint']

"""Auto-generated package __init__.py"""
from .aurapaint import *  # noqa: F401, F403
