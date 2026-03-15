"""
process_manager.py — backward-compat shim.
Real implementation lives in process_manager/ package.
"""

from process_manager.s_round import *  # noqa
from process_manager.SigmaProcessManager import *  # noqa

__all__ = ['s_round', 'SigmaProcessManager']
