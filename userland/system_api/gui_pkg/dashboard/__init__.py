"""
dashboard.py — backward-compat shim.
Real implementation lives in dashboard/ package.
"""

from .dashboard.DashboardPage import *  # noqa

__all__ = ['DashboardPage']

"""Auto-generated package __init__.py"""
from .dashboardpage import *  # noqa: F401, F403
