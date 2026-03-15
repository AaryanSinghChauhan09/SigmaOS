"""
monitor.py — backward-compat shim.
Real implementation lives in monitor/ package.
"""

from monitor.SigmaWorkstationMonitor import *  # noqa

__all__ = ['SigmaWorkstationMonitor']
