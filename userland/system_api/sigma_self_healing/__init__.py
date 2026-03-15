"""
sigma_self_healing.py — backward-compat shim.
Real implementation lives in sigma_self_healing/ package.
"""

from .sigma_self_healing.SigmaFixOrchestrator import *  # noqa

__all__ = ['SigmaFixOrchestrator']

"""Auto-generated package __init__.py"""
from .sigmafixorchestrator import *  # noqa: F401, F403
