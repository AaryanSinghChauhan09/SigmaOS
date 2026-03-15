"""
chess.py — backward-compat shim.
Real implementation lives in chess/ package.
"""

from .chess.SovereignStrategist import *  # noqa

__all__ = ['SovereignStrategist']

"""Auto-generated package __init__.py"""
from .sovereignstrategist import *  # noqa: F401, F403
