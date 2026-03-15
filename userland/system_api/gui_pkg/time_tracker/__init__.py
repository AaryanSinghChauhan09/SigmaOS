"""
time_tracker.py — backward-compat shim.
Real implementation lives in time_tracker/ package.
"""

from .time_tracker.TimeTrackerPage import *  # noqa

__all__ = ['TimeTrackerPage']

"""Auto-generated package __init__.py"""
from .timetrackerpage import *  # noqa: F401, F403
