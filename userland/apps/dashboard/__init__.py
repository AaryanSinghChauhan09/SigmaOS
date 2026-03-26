"""
dashboard.py — backward-compat shim.
Real implementation lives in dashboard/ package.
"""

from .dashboard.MorphicDashboard import *  # noqa

__all__ = ['MorphicDashboard']

"""Auto-generated package __init__.py"""
from .morphicdashboard import *  # noqa: F401, F403
