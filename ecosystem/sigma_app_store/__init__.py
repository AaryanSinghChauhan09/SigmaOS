"""
sigma_app_store.py — backward-compat shim.
Real implementation lives in sigma_app_store/ package.
"""

from .sigma_app_store.SigmaAppStore import *  # noqa

__all__ = ['SigmaAppStore']

"""Auto-generated package __init__.py"""
from .sigmaappstore import *  # noqa: F401, F403
