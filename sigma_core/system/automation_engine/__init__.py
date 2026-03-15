"""
automation_engine.py — backward-compat shim.
Real implementation lives in automation_engine/ package.
"""

from .automation_engine.AutomationEngine import *  # noqa

__all__ = ['AutomationEngine']

"""Auto-generated package __init__.py"""
from .automationengine import *  # noqa: F401, F403
