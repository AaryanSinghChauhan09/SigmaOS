"""
memory_manager.py — backward-compat shim.
Real implementation lives in memory_manager/ package.
"""

from memory_manager.SigmaMemoryManager import *  # noqa

__all__ = ['SigmaMemoryManager']
