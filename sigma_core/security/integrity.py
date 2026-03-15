"""
integrity.py — backward-compat shim.
Real implementation lives in integrity/ package.
"""

from integrity.IntegrityGuard import *  # noqa

__all__ = ['IntegrityGuard']
