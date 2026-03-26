"""
shell_forge.py — backward-compat shim.
Real implementation lives in shell_forge/ package.
"""

from .shell_forge.ShellForge import *  # noqa

__all__ = ['ShellForge']

"""Auto-generated package __init__.py"""
from .shellforge import *  # noqa: F401, F403
