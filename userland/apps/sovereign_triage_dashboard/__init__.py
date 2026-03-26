"""
sovereign_triage_dashboard.py — backward-compat shim.
Real implementation lives in sovereign_triage_dashboard/ package.
"""

from .sovereign_triage_dashboard.TriageDashboard import *  # noqa

__all__ = ['TriageDashboard']

"""Auto-generated package __init__.py"""
from .triagedashboard import *  # noqa: F401, F403
