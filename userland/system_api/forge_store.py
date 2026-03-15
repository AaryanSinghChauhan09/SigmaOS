"""
forge_store.py — backward-compat shim.
Real implementation lives in forge_store/ package.
"""

from forge_store.SigmaForgeStore import *  # noqa

__all__ = ['SigmaForgeStore']
