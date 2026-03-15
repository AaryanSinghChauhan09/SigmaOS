"""
integrity.py — backward-compat shim.
Real implementation lives in integrity/ package.
"""

from .integrity.IntegrityGuard import *  # noqa

__all__ = ['IntegrityGuard']

"""Auto-generated package __init__.py"""
from .integrityguard import *  # noqa: F401, F403
