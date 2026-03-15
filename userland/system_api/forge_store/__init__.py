"""
forge_store.py — backward-compat shim.
Real implementation lives in forge_store/ package.
"""

from .forge_store.SigmaForgeStore import *  # noqa

__all__ = ['SigmaForgeStore']

"""Auto-generated package __init__.py"""
from .sigmaforgestore import *  # noqa: F401, F403
