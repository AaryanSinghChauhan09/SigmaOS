"""
sigma_auditor.py — backward-compat shim.
Real implementation lives in sigma_auditor/ package.
"""

from sigma_auditor.SigmaAuditor import *  # noqa

__all__ = ['SigmaAuditor']
