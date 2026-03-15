"""
omni_purge.py — backward-compat shim.
Real implementation lives in omni_purge/ package.
"""

from .omni_purge.OmniPurge import *  # noqa

__all__ = ['OmniPurge']

"""Auto-generated package __init__.py"""
from .omnipurge import *  # noqa: F401, F403
