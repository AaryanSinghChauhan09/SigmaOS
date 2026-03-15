"""
stopwatch_timer.py — backward-compat shim.
Real implementation lives in stopwatch_timer/ package.
"""

from stopwatch_timer.SigmaStopwatch import *  # noqa
from stopwatch_timer.launch import *  # noqa

__all__ = ['SigmaStopwatch', 'launch']
