"""
process_manager.py — backward-compat shim.
Real implementation lives in process_manager/ package.
"""

from .process_manager.SigmaProcessManager import *  # noqa

__all__ = ['SigmaProcessManager']

"""Auto-generated package __init__.py"""
from .sigmaprocessmanager import *  # noqa: F401, F403
