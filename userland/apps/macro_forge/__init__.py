"""
macro_forge.py — backward-compat shim.
Real implementation lives in macro_forge/ package.
"""

from .macro_forge.MacroForge import *  # noqa

__all__ = ['MacroForge']

"""Auto-generated package __init__.py"""
from .macroforge import *  # noqa: F401, F403
