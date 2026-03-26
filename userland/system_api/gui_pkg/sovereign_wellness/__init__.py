"""
sovereign_wellness.py — backward-compat shim.
Real implementation lives in sovereign_wellness/ package.
"""

from .sovereign_wellness.SovereignWellnessPage import *  # noqa

__all__ = ['SovereignWellnessPage']

"""Auto-generated package __init__.py"""
from .sovereignwellnesspage import *  # noqa: F401, F403
