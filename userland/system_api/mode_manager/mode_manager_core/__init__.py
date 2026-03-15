"""
mode_manager_core.py — backward-compat shim.
Real implementation lives in mode_manager_core/ package.
"""

from .mode_manager_core.SigmaModeManager import *  # noqa

__all__ = ['SigmaModeManager']

"""Auto-generated package __init__.py"""
from .sigmamodemanager import *  # noqa: F401, F403
