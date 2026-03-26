"""
bash.py — backward-compat shim.
Real implementation lives in bash/ package.
"""

from .bash.SovereignShell import *  # noqa

__all__ = ['SovereignShell']

"""Auto-generated package __init__.py"""
from .sovereignshell import *  # noqa: F401, F403
