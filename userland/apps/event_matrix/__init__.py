"""
event_matrix.py — backward-compat shim.
Real implementation lives in event_matrix/ package.
"""

from .event_matrix.EventMatrix import *  # noqa

__all__ = ['EventMatrix']

"""Auto-generated package __init__.py"""
from .eventmatrix import *  # noqa: F401, F403
