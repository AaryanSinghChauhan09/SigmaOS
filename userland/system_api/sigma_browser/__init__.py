"""
sigma_browser.py — backward-compat shim.
Real implementation lives in sigma_browser/ package.
"""

from .sigma_browser.SigmaOmniBrowser import *  # noqa

__all__ = ['SigmaOmniBrowser']

"""Auto-generated package __init__.py"""
from .sigmaomnibrowser import *  # noqa: F401, F403
