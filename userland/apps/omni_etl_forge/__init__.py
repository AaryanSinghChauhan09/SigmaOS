"""
omni_etl_forge.py — backward-compat shim.
Real implementation lives in omni_etl_forge/ package.
"""

from .omni_etl_forge.OmniETLForge import *  # noqa

__all__ = ['OmniETLForge']

"""Auto-generated package __init__.py"""
from .omnietlforge import *  # noqa: F401, F403
