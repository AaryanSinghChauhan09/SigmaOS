"""
ncert_master_lab.py — backward-compat shim.
Real implementation lives in ncert_master_lab/ package.
"""

from .ncertmasterlab.ncert_master_lab import NCERTMasterLab  # type: ignore

__all__ = ['NCERTMasterLab']
