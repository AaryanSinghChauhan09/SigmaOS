"""
stopwatch_timer.py — backward-compat shim.
Real implementation lives in stopwatch_timer/ package.
"""

from .stopwatch_timer.SigmaStopwatch import *  # noqa
from .stopwatch_timer.launch import *  # noqa

__all__ = ['SigmaStopwatch', 'launch']

"""Auto-generated package __init__.py"""
from .sigmastopwatch import *  # noqa: F401, F403
from .launch import *  # noqa: F401, F403
