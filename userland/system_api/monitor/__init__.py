"""
monitor.py — backward-compat shim.
Real implementation lives in monitor/ package.
"""

from .monitor.SigmaWorkstationMonitor import *  # noqa

__all__ = ['SigmaWorkstationMonitor']

"""Auto-generated package __init__.py"""
from .sigmaworkstationmonitor import *  # noqa: F401, F403
