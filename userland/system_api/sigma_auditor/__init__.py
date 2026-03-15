"""
sigma_auditor.py — backward-compat shim.
Real implementation lives in sigma_auditor/ package.
"""

from .sigma_auditor.SigmaAuditor import *  # noqa

__all__ = ['SigmaAuditor']

"""Auto-generated package __init__.py"""
from .sigmaauditor import *  # noqa: F401, F403
