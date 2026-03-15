"""
memory_manager.py — backward-compat shim.
Real implementation lives in memory_manager/ package.
"""

from .memory_manager.SigmaMemoryManager import *  # noqa

__all__ = ['SigmaMemoryManager']

"""Auto-generated package __init__.py"""
from .sigmamemorymanager import *  # noqa: F401, F403
