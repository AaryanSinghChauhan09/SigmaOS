"""
guide_apex.py — backward-compat shim.
Real implementation lives in guide_apex/ package.
"""

from .guide_apex.GuideApex import *  # noqa

__all__ = ['GuideApex']

"""Auto-generated package __init__.py"""
from .guideapex import *  # noqa: F401, F403
