"""
sigma_automation_hub.py — backward-compat shim.
Real implementation lives in sigma_automation_hub/ package.
"""

from sigma_automation_hub.SigmaOmniAutomator import *  # noqa

__all__ = ['SigmaOmniAutomator']
