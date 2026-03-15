"""
_build_perf_status.py — backward-compat shim.
Real implementation lives in _build_perf_status/ package.
"""

from ._build_perf_status.SigmaGUI import *  # noqa

__all__ = ['SigmaGUI']

"""Auto-generated package __init__.py"""
from .sigmagui import *  # noqa: F401, F403
