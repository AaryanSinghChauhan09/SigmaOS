"""
support_ecosystem.py — backward-compat shim.
Real implementation lives in support_ecosystem/ package.
"""

from .support_ecosystem.SigmaSupportEcosystem import *  # noqa

__all__ = ['SigmaSupportEcosystem']

"""Auto-generated package __init__.py"""
from .sigmasupportecosystem import *  # noqa: F401, F403
