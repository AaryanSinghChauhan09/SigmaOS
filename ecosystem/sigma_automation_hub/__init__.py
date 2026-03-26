"""
sigma_automation_hub.py — backward-compat shim.
Real implementation lives in sigma_automation_hub/ package.
"""

from .sigma_automation_hub.SigmaOmniAutomator import *  # noqa

__all__ = ['SigmaOmniAutomator']

"""Auto-generated package __init__.py"""
from .sigmaomniautomator import *  # noqa: F401, F403
