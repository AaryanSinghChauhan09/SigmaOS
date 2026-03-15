"""
omni_savant.py — backward-compat shim.
Real implementation lives in omni_savant/ package.
"""

from .omni_savant.OmniSavant import *  # noqa

__all__ = ['OmniSavant']

"""Auto-generated package __init__.py"""
from .omnisavant import *  # noqa: F401, F403
