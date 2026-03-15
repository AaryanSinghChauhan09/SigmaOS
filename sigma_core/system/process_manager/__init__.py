"""
process_manager.py — backward-compat shim.
Real implementation lives in process_manager/ package.
"""

from .process_manager.s_round import *  # noqa
from .process_manager.SigmaProcessManager import *  # noqa

__all__ = ['s_round', 'SigmaProcessManager']

"""Auto-generated package __init__.py"""
from .s_round import *  # noqa: F401, F403
from .sigmaprocessmanager import *  # noqa: F401, F403
