"""
mode_manager.py — backward-compat shim.
Real implementation lives in mode_manager/ package.
"""

from .mode_manager.SigmaModeManager import *  # noqa

__all__ = ['SigmaModeManager']

"""Auto-generated package __init__.py"""
from .sigmamodemanager import *  # noqa: F401, F403
